# Checking or building examples fails
```bash
$ cargo +nightly build --examples
   Compiling example_crate v0.1.0 (.../example_crate)
warning: unused attribute
 --> examples/b.rs:1:1
  |
1 | #![feature(async_await, futures_api)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_attributes)] on by default

warning: crate-level attribute should be in the root module
 --> examples/b.rs:1:1
  |
1 | #![feature(async_await, futures_api)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
```
