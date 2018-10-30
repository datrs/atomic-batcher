extern crate atomic_batcher;
extern crate tokio;

use atomic_batcher::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

#[test]
fn run_once() {
  fn run(val: Vec<u64>, _batcher: Batcher<u64>) -> () {
    assert_eq!(val, vec![1, 2, 3]);
  };
  let batcher = Batcher::new(Arc::new(run));
  batcher.append(vec![1, 2, 3], None);
}

#[test]
fn run_with_done() {
  let run = |val: Vec<u64>, batcher: Batcher<u64>| -> () {
    if val == vec![1, 2, 3] {
      batcher.append(vec![4, 5, 6], None);
      batcher.done(Ok(()));
    } else {
      assert_eq!(val, vec![4, 5, 6]);
    }
  };
  let batcher = Batcher::new(Arc::new(run));
  batcher.append(vec![1, 2, 3], None);
}

#[test]
fn run_with_callback() {
  let run = |val: Vec<u64>, batcher: Batcher<u64>| -> () {
    if val == vec![1, 2, 3] {
      batcher.done(Err("some wrong"));
    } else {
      assert_eq!(val, vec![]);
    }
  };
  let batcher = Batcher::new(Arc::new(run));
  batcher.append(
    vec![1, 2, 3],
    Some(Arc::new(move |err| {
      if let Err(s) = err {
        assert_eq!(s, "some wrong");
      }
    })),
  );
}

#[test]
fn run_async() {
  let when = Instant::now() + Duration::from_millis(1000);
  let run = move |val: Vec<u64>, _batcher: Batcher<u64>| -> () {
    if val != vec![1, 2, 3] {
      assert_eq!(val, vec![4, 5, 6, 7, 8, 9]);
    }
  };
  let batcher = Batcher::new(Arc::new(run));
  batcher.append(vec![1, 2, 3], None);
  batcher.append(vec![4, 5, 6], None);
  batcher.append(vec![7, 8, 9], None);
  let task = Delay::new(when)
    .and_then(|_| {
      batcher.done(Ok(()));
      Ok(())
    })
  tokio::run(task);
}
