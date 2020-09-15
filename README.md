# Weblog

weblog is a crate that defines a set of macros for calling `console.log()`, `console.error()` and other members of the browser's console API when targeting Wasm.

## Features

* Supports `web-sys` and `stdweb` backends with an identical public API
* Support for variadic arguments on all calls
* No stringification before sending to the browser - log entire objects and use the full introspective debugging power of the browser console.

## Examples

A simple example.

```rust
console_log!("Hello world!");
```

Passing multiple arguments is fine too.

```rust
console_log!("Foo", "bar", "baz");
```

All of the common browser log levels are supported.

```rust
console_debug!("Just testing...");
console_warn!("...but then...");
console_error!("...something bad happened.");
```

It's possible to send more than just strings or `&str`s:

```rust
console_log!(
    "&str",
    "string".to_string(),
    1,
    2.0,
    3f32,
    true,
    false
);
```

When using `web-sys` crate the macros accept any value that implements the `Into<JsValue>` trait. See [JsValue](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html) for
more details.

No stringification is performed on the Rust side - so objects will be fully introspectable in the browser's console!


## Usage

```toml
# Defaults to web-sys
weblog = "0.1"

# For stdweb:
weblog = { version = "0.1", default-features = false, features = ["std_web"] }
```

See the documentation for usage examples.

The crate currently exposes the following macros:

* `console_clear!`
* `console_debug!`
* `console_dir!`
* `console_dirxml!`
* `console_error!`
* `console_info!`
* `console_log!`
* `console_trace!`
* `console_warn!`

## License

Licensed under MIT or Apache-2.

