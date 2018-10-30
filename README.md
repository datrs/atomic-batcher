# atomic-batcher 

[![crates.io version][1]][2] [![build status][3]][4] [![downloads][5]][6]

`atomic-batcher` is a simple batching function that allows you to atomically batch a series of operations. Adapted from [mafintosh/atomic-batcher](https://github.com/mafintosh/atomic-batcher)

- Documetaion
- Crates.io

## Installation

```sh
cargo add atomic-batcher
```

Install [cargo-edit](https://github.com/killercup/cargo-edit) to extend Cargo, allowing you to add, remove, and upgrade dependencies by modifying your Cargo.toml file from the command line.

## Usage

```rust
extern crate atomic_batcher;
extern crate tokio;

use atomic_batcher::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

fn main() {
  let when = Instant::now() + Duration::from_millis(2000);
  let run = move |val: Vec<u64>, _batcher: Batcher<u64>| -> () {
    println!("{:?}", val);  
  };
  
  // Create a batcher with a run function which will be called  
  // when batcher's inner state `running` is OFF and inner state `pending_batch`
  // is not empty.
  let batcher = Batcher::new(Arc::new(run));

  // Before this first append, batcher's inner state `running` is initial OFF, 
  // so batcher will call the run function with the append value directly,
  // then inner state `running` is ON.  
  batcher.append(vec![1, 2, 3], None);

  // Now because inner state `running` is ON, run function won't be called.
  // But the data `vec![4, 5, 6]` and `vec![7, 8, 9]` will be pushed to 
  // batcher's `pending_batch`.  
  batcher.append(vec![4, 5, 6], None);
  batcher.append(vec![7, 8, 9], None);

  // Now `pending_batch` is vec![4, 5, 6, 7, 8, 9].
  // After 2 seconds, batcher.done get called which will turn `running` to OFF,
  // then call run function with `pending_batch`.
  // Finally turn `running` to ON again.  
  let task = Delay::new(when)
  .and_then(|_| {
    batcher.done(Ok(()));
    Ok(())
  })
  .map_err(|e| panic!("delay errored; err={:?}", e));
  tokio::run(task);
}
```

Running the above example will print

```sh
[1, 2, 3]

// two seconds later
[4, 5, 6, 7, 8, 9]
```

## License
Licensed under the AGPL-3.0+. See [LICENSE](./LICENSE).

[1]: https://img.shields.io/crates/v/atomic-batcher.svg?style=flat-square
[2]: https://crates.io/crates/atomic-batcher
[3]: https://img.shields.io/travis/Zhouhansen/atomic-batcher.svg?style=flat-square
[4]: https://travis-ci.org/Zhouhansen/atomic-batcher
[5]: https://img.shields.io/crates/d/atomic-batcher.svg?style=flat-square
[6]: https://crates.io/crate/atomic-batcher