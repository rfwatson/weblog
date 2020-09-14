# Weblog

weblog is a crate that defines a set of macros for calling `console.log()`, `console.error()` and other members of the browser's console API when targeting Wasm.

### Features

* Supports `web-sys` and `stdweb` backends with an identical public API
* Support for variadic arguments on all calls
* No stringification before sending to the browser - log entire objects and use the full introspective debugging power of the browser console.

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

