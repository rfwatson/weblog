//! weblog is a crate that defines a set of macros for calling `console.log()`, `console.error()`
//! and other members of the browser's console API when targeting Wasm.
//!
//! # Features
//!
//! * Supports `web-sys` and `stdweb` backends with an identical public API
//! * Support for variadic arguments on all calls
//! * No stringification before sending to the browser - log entire objects and use the full
//! introspective debugging power of the browser console.
//!
//! # Examples
//!
//! A simple example.
//!
//! ```
//! # #[macro_use] extern crate weblog;
//! # fn main() {
//! console_log!("Hello world!");
//! # }
//! ```
//! Passing multiple arguments is fine too.
//!
//! ```
//! # #[macro_use] extern crate weblog;
//! # fn main() {
//! console_log!("Foo", "bar", "baz");
//! # }
//! ```
//! All of the common browser log levels are supported.
//!
//! ```
//! # #[macro_use] extern crate weblog;
//! # fn main() {
//! console_debug!("Just testing...");
//! console_warn!("...but then...");
//! console_error!("...something bad happened.");
//! # }
//! ```
//! It's possible to send more than just strings or `&str`s:
//!
//! ```
//! # #[macro_use] extern crate weblog;
//! # fn main() {
//! console_log!(
//!     "&str",
//!     "string".to_string(),
//!     1,
//!     2.0,
//!     3f32,
//!     true,
//!     false
//! );
//! # }
//! ```
//! When using `web-sys` crate the macros accept any value that implements the `Into<JsValue>` trait. See [JsValue](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html) for
//! more details.
//!
//! No stringification is performed on the Rust side - so objects will be fully introspectable in
//! the browser's console!
//!
//! # Usage
//!
//! By default, the crate assumes the presence of the `web-sys` crate.
//!
//! ```toml
//! weblog = "0.1"
//! ```
//!
//!
//! If you'd prefer to use it
//! with `stdweb`, enable the feature in `Cargo.toml`:
//!
//! ```toml
//! weblog = { version = "0.1", default-features = false, features = ["stdweb"] }
//! ```
//!
mod console;

#[cfg(feature = "web_sys")]
pub use self::console::web_sys::*;

#[cfg(feature = "std_web")]
pub use self::console::std_web::*;
