#![warn(unsafe_code, missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! A simple crate that allows to easily and atomically batch similar operations.

//! ## Example
//! ```rust
//! use atomic_batcher::Batcher;
//!
//! fn main() {
//!   let say_name = |names| async move {
//!     for name in &names {
//!       println!("My name is {}.", name);
//!     }
//!     return names;
//!   };
//!
//!   let say_hello = |names| async move {
//!     for name in names {
//!       println!("Hello {}!", name)
//!     }
//!   };
//!
//!   let batch = Batcher::new(say_name);
//!   batch.appendcb("Ferris", say_hello);
//!   batch.append("Rustacean");
//! }
//! ```
//! Outputs:
//! ```text
//! My name is Ferris.
//! Hello Ferris!
//! My name is Rustacean.
//! ```

use futures::executor::block_on;
use std::future::Future;

/// Stores a function and schedules new operations against it as new values are appended.
pub struct Batcher<INPUT, FUTURE: Future<Output = RESULT>, RESULT> {
  run: fn(Vec<INPUT>) -> FUTURE,
}

impl<INPUT, FUTURE: Future<Output = RESULT>, RESULT>
  Batcher<INPUT, FUTURE, RESULT>
{
  /// Creates a new batcher for the `function` passed as an argument.
  ///
  /// The `function`'s expected `INPUT` and `OUTPUT` determines the batcher's type.
  pub fn new(
    function: fn(Vec<INPUT>) -> FUTURE,
  ) -> Batcher<INPUT, FUTURE, RESULT> {
    Batcher { run: function }
  }

  /// Schedules a new operation passing `value` as the only argument.
  pub fn append(&self, value: INPUT) {
    block_on(async {
      (self.run)(vec![value]).await;
    });
  }

  /// Schedules a new operation passing `value` as the only argument.
  ///
  /// Afterwards `callback` will be run passing the result of the operation as the only argument.
  pub fn appendcb<CB: Future>(&self, value: INPUT, callback: fn(RESULT) -> CB) {
    block_on(async {
      let result = (self.run)(vec![value]).await;
      callback(result).await;
    });
  }
}
