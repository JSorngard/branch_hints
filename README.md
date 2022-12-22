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

# Credit
The functions are taken directly from the [hashbrown](https://crates.io/crates/hashbrown) crate and this crate simply exposes them, all credit belongs to the hashbrown authors.
