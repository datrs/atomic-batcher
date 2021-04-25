# atomic-batcher

[![crates.io version][1]][2] [![build status][3]][4] [![downloads][5]][6]

`atomic-batcher` is a simple batching function that allows you to atomically batch a series of operations. Adapted from [mafintosh/atomic-batcher](https://github.com/mafintosh/atomic-batcher)

- [Documetaion](https://docs.rs/atomic-batcher)
- [Crates.io](https://crates.io/crate/atomic-batcher)

## Installation

```sh
cargo add atomic-batcher
```

Install [cargo-edit](https://github.com/killercup/cargo-edit) to extend Cargo, allowing you to add, remove, and upgrade dependencies by modifying your Cargo.toml file from the command line.

## Usage

```rust
extern crate atomic_batcher;

use atomic_batcher::*;

fn main() {
  let run = |val: Vec<Vec<u64>>| async move {
    let flat: Vec<u64> = val.into_iter().flatten().collect();
    println!("{:?}", flat);
  };
  
  // Create a batcher with a run function
  let batcher = Batcher::new(run);

  // Batcher will call the run function with the append value directly,
  batcher.append(vec![1, 2, 3]);

  // With current implementation batcher will call the run
  // function once for each append operation independently.
  // This contrasts with the original nodejs implementation,
  // but it shouldn't break compatibility since the original also claims:
  // `Only one batch is guaranteed to be run at the time.`
  // This can possibly be improved using Future combinators.
  batcher.append(vec![4, 5, 6]);
  batcher.append(vec![7, 8, 9]);
}
```

Running the above example will print

```sh
[1, 2, 3]
[4, 5, 6]
[7, 8, 9]
```

## License
[MIT](./LICENSE-MIT) OR [APACHE](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/atomic-batcher.svg?style=flat-square
[2]: https://crates.io/crates/atomic-batcher
[3]: https://api.travis-ci.org/datrs/atomic-batcher.svg?branch=master
[4]: https://travis-ci.org/datrs/atomic-batcher
[5]: https://img.shields.io/crates/d/atomic-batcher.svg?style=flat-square
[6]: https://crates.io/crate/atomic-batcher