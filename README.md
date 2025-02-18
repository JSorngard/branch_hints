# branch_hints

[![Crates.io Version](https://img.shields.io/crates/v/branch_hints?logo=rust)](https://crates.io/crates/branch_hints)
[![docs.rs](https://img.shields.io/docsrs/branch_hints?logo=docsrs)](https://docs.rs/branch_hints/latest/branch_hints/)
[![Static Badge](https://img.shields.io/badge/github-JSorngard%2Fbranch__hints-8da0cb?logo=github)](https://github.com/JSorngard/branch_hints)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/branch_hints/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/branch_hints/actions/workflows/rust.yml)

This crate provides the functions `likely` and `unlikely` that work as compiler hints for branching.

These functions enable constructions like

```rust
if likely(condition) {
    // main code
} else {
    // error handling code
}
```

and they should be optimized away by the compiler.

## Credit

The functions are taken directly from the [`hashbrown`](https://crates.io/crates/hashbrown) crate and this crate simply exposes them, all credit belongs to the hashbrown authors.

## Note

This is a very minimal crate. If you want more comprehensive functionality, take a look at the [`likely_stable`](https://crates.io/crates/likely_stable) crate.

Verify for yourself with benchmarks if these functions do anything for you,
as even the unstable intrinsics sometimes don't do anything, and may even reduce performance: https://github.com/rust-lang/rust/issues/88767.
This implementation of them was removed from `hashbrown` for this reason: https://github.com/rust-lang/hashbrown/issues/482.


<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
