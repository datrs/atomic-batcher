#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! extern crate atomic_batcher;
//! extern crate tokio;

//! use atomic_batcher::*;
//! use std::sync::Arc;
//! use std::time::{Duration, Instant};
//! use tokio::prelude::*;
//! use tokio::timer::Delay;

//! fn main() {
//!   let when = Instant::now() + Duration::from_millis(2000);
//!   let run = move |val: Vec<u64>, _batcher: &Batcher<u64>| -> () {
//!     println!("{:?}", val);  
//!   };
//!
//!   // Create a batcher with a run function which will be called  
//!   // when batcher's inner state `running` is OFF and inner state `pending_batch`
//!   // is not empty.
//!   let batcher = Batcher::new(Box::new(run));
//!
//!   // Before this first append, batcher's inner state `running` is initial OFF,
//!   // so batcher will call the run function with the append value directly,
//!   // then inner state `running` is ON.
//!   batcher.append(vec![1, 2, 3], None);
//!
//!   // Now because inner state `running` is ON, run function won't be called.
//!   // But the data `vec![4, 5, 6]` and `vec![7, 8, 9]` will be pushed to
//!   // batcher's `pending_batch`.
//!   batcher.append(vec![4, 5, 6], None);
//!   batcher.append(vec![7, 8, 9], None);
//!
//!   // Now `pending_batch` is vec![4, 5, 6, 7, 8, 9].
//!   // After 2 seconds, batcher.done get called which will turn `running` to OFF,
//!   // then call run function with `pending_batch`.
//!   // Finally turn `running` to ON again.
//!   let task = Delay::new(when)
//!   .and_then(move |_| {
//!     batcher.done(Ok(()));
//!     Ok(())
//!   })
//!   .map_err(|e| panic!("delay errored; err={:?}", e));
//!   tokio::run(task);
//! }
//! ```
//! Running the above example will print
//! ```sh
//! [1, 2, 3]
//!
//! // two seconds later
//! [4, 5, 6, 7, 8, 9]
//! ```
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

type Cb = Box<Fn(Result<(), &str>) -> () + Send + Sync>;
/// Describing optional batched callback function
pub type CbOption = Option<Cb>;

/// Batching representation.
pub struct Batcher<T> {
  running: AtomicBool,
  pending_batch: Mutex<Vec<T>>,
  pending_callbacks: Mutex<Vec<Cb>>,
  callbacks: Mutex<Vec<Cb>>,
  run: Box<Fn(Vec<T>, &Batcher<T>) -> () + Send + Sync>,
}

impl<T> Batcher<T> {
  /// Create a new batcher with a run function.
  pub fn new(run: Box<Fn(Vec<T>, &Batcher<T>) -> () + Send + Sync>) -> Arc<Self> {
    Arc::new(Batcher {
      running: AtomicBool::new(false),
      pending_batch: Mutex::new(Vec::new()),
      pending_callbacks: Mutex::new(Vec::new()),
      callbacks: Mutex::new(Vec::new()),
      run,
    })
  }
  /// Accept an array of values and a callback.
  /// The accepted callback is called when the batch containing the values have been run.
  pub fn append(&self, val: Vec<T>, cb: CbOption) -> () {
    if self.running.load(Ordering::Relaxed) {
      if self.pending_batch.lock().unwrap().len() == 0 {
        *self.pending_callbacks.lock().unwrap() = Vec::new();
      }
      self.pending_batch.lock().unwrap().extend(val);
      if let Some(cb) = cb {
        self.callbacks.lock().unwrap().push(cb);
      }
    } else {
      if let Some(cb) = cb {
        *self.callbacks.lock().unwrap() = vec![cb];
      }
      self.running.store(true, Ordering::Relaxed);
      (self.run)(val, self);
    }
  }
  /// Turn batcher's running state to off. then call the run function.
  pub fn done(&self, err: Result<(), &str>) -> () {
    for cb in self.callbacks.lock().unwrap().iter() {
      cb(err)
    }
    self.running.store(false, Ordering::Relaxed);
    let mut pending_callbacks = self.pending_callbacks.lock().unwrap();
    let mut pending_batch = self.pending_batch.lock().unwrap();
    *self.callbacks.lock().unwrap() = pending_callbacks.drain(..).collect();
    let nextbatch: Vec<T> = pending_batch.drain(..).collect();
    if nextbatch.is_empty() && self.callbacks.lock().unwrap().is_empty() {
      return;
    }
    self.running.store(true, Ordering::Relaxed);
    (self.run)(nextbatch, self);
  }
}
