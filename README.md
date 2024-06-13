# branch_hints
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
The functions are taken directly from the [hashbrown](https://crates.io/crates/hashbrown) crate and this crate simply exposes them, all credit belongs to the hashbrown authors.

## Note
This is a very minimal crate. If you want more comprehensive functionality, take a look at the [likely_stable](https://crates.io/crates/likely_stable) crate.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](https://github.com/JSorngard/branch_hints/blob/main/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)  
 * MIT license
   ([LICENSE-MIT](https://github.com/JSorngard/branch_hints/blob/main/LICENCE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
