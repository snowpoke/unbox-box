# unbox-box

This crate provides `.unbox_ref()` and `.unbox_mut()` methods on `Box` types. They are equivalent to `.deref()` and `.deref_mut()`, but make it easier to see that the variable that is being operated on is of type `Box`.


## Usage
The idea for this crate came to me this morning when I woke up to this code
```rust
let expr: &syn::ExprLit = match &*assignment.expr { ... };
```
and was very unsure as to why there was a deref operator `*` in front of my variable. Now I just import
```toml
unbox-box = "0.1"
```
and I can replace this code with
```rust
use unbox_box::BoxExt as _;
let expr: &syn::ExprLit = match assignment.expr.unbox_ref() { ... };
```

When I wake up to this code tomorrow, I'm pretty sure I'll recognize that `assignment.expr` is a `Box`. Wish me luck!


## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.