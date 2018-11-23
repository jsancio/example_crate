# Checking or building examples fails
```bash
$ cargo +nightly check --examples
    Checking example_crate v0.1.0 (.../example_crate)
error[E0432]: unresolved import `b`
 --> examples/c.rs:1:5
  |
1 | use b::b;
  |     ^ Could not find `b` in `{{root}}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: Could not compile `example_crate`.

To learn more, run the command again with --verbose.
```


```bash
$ cargo +nightly build --examples
   Compiling example_crate v0.1.0 (.../example_crate)
error[E0432]: unresolved import `b`
 --> examples/c.rs:1:5
  |
1 | use b::b;
  |     ^ Could not find `b` in `{{root}}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: Could not compile `example_crate`.

To learn more, run the command again with --verbose.
```

It looks like cargo is creating `libb`:
```bash
$ tree target/debug/examples/
target/debug/examples/
├ b-1dcc503f23e0b76b.d
├ b-7b617d848b897e49.d
├ b-e7c00f0c42ff9607.d
├ c-4cad30ab9241529b.d
├ c-ecabb81ac2bff21c.d
├ libb-1dcc503f23e0b76b.rmeta
├ libb-7b617d848b897e49.rlib
├ libb.d
├ libb-e7c00f0c42ff9607.rmeta
├ libb.rlib
└ libb.rmeta

0 directories, 11 files
```
