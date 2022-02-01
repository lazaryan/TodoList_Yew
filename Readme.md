# TodoList in Rust (Yew frameork)

This is an implementation of Todo for Yew.

This stores the full state of the model, including: all entries, entered text and chosen filter.

## Gettong start

Install to your system `Rust compilation`

In terminal:

```bash
$ cargo install --locked trunk
$ cargo install wasm-bindgen-cli
```

## Build project

The Trunk CLI provides several useful commands but during development `trunk serve` is certainly the most useful one. It runs a local server for you and automatically rebuilds the app when it detects changes.

When you're ready to release your app, you can just run `trunk build --release`.
