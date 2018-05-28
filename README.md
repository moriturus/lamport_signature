# `lamport_signature`

[![Crates.io](https://img.shields.io/crates/v/lamport_signature.svg)](https://crates.io/crates/lamport_signature)
[![docs.rs](https://docs.rs/lamport_signature/badge.svg)](https://docs.rs/lamport_signature)
[![Build Status](https://travis-ci.org/moriturus/lamport_signature.svg?branch=master)](https://travis-ci.org/moriturus/lamport_signature)
[![GitHub license](https://img.shields.io/github/license/moriturus/lamport_signature.svg)](https://github.com/moriturus/lamport_signature/blob/master/LICENSE)

*lamport_signature* is a Rust implementation of the [Lamport one-time signature scheme](https://en.wikipedia.org/wiki/Lamport_signature).

## Difference from the [lamport_sigs](https://github.com/SpinResearch/lamport_sigs.rs) crate

- *lamport_signature* is strongly influenced by the *lamport_sigs* crate but **NO CODES ARE SHARED OR COPIED**.
- *lamport_signature* can use arbitrary fixed output size digest algorithm implemented in [RustCrypto/hashes](https://github.com/RustCrypto/hashes).
- *lamport_signature* can use arbitrary RNG (*Random Number Generator*) implemented in [rust-lang-nursery/rand](https://github.com/rust-lang-nursery/rand).

## Documentation

Documentation is [available](https://docs.rs/lamport_signature).

## Usage

```rust
extern crate lamport_signature;
extern crate sha2;
extern crate rand;

use lamport_signature::{PublicKey, PrivateKey, generate_keys};
use sha2::Sha256;
use rand::thread_rng;

let mut rng = thread_rng();
let (mut private_key, public_key) = generate_keys::<Sha256, _>(&mut rng);

let signature = private_key.sign(b"Hello, World!").expect("signing failed");

assert!(public_key.verify(&signature, b"Hello, World!"));
```

## Bug Reporting

Please report bugs either as pull requests or as issues in [the issue
tracker](https://github.com/moriturus/lamport_signature). *lamport_signature* has a
**full disclosure** vulnerability policy. **Please do NOT attempt to report
any security vulnerability in this code privately to anybody.**

## License

[MIT License](LICENSE).