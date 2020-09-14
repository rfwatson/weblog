#[doc(hidden)]
pub extern crate web_sys;

use paste::paste;

macro_rules! websys_rules {
    ($level:ident) => {
        paste! {
            #[doc = "Call the browser's `console." $level "()` function.\n\n\
            The web-sys crate accepts up to a maximum of seven arguments, all of which must implement `Into<JsValue>`.\n\n\
            See the [wasm-bindgen documentation](../wasm_bindgen/struct.JsValue.html) for more information."]
            #[macro_export]
            macro_rules! [<console_ $level>] {
                () => {
                    $crate::web_sys::console::[<$level _0>]()
                };
                ($a:expr) => {
                    $crate::web_sys::console::[<$level _1>](&$a.into())
                };
                ($a:expr, $b:expr) => {
                    $crate::web_sys::console::[<$level _2>](&$a.into(), &$b.into())
                };
                ($a:expr, $b:expr, $c:expr) => {
                    $crate::web_sys::console::[<$level _3>](&$a.into(), &$b.into(), &$c.into())
                };
                ($a:expr, $b:expr, $c:expr, $d:expr) => {
                    $crate::web_sys::console::[<$level _4>](&$a.into(), &$b.into(), &$c.into(), &$d.into())
                };
                ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
                    $crate::web_sys::console::[<$level _5>](
                        &$a.into(),
                        &$b.into(),
                        &$c.into(),
                        &$d.into(),
                        &$e.into(),
                    )
                };
                ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
                    $crate::web_sys::console::[<$level _6>](
                        &$a.into(),
                        &$b.into(),
                        &$c.into(),
                        &$d.into(),
                        &$e.into(),
                        &$f.into(),
                    )
                };
                ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
                    $crate::web_sys::console::[<$level _7>](
                        &$a.into(),
                        &$b.into(),
                        &$c.into(),
                        &$d.into(),
                        &$e.into(),
                        &$f.into(),
                        &$g.into(),
                    )
                };
            }
        }
    }
}

websys_rules!(debug);
websys_rules!(dir);
websys_rules!(dirxml);
websys_rules!(error);
websys_rules!(info);
websys_rules!(log);
websys_rules!(trace);
websys_rules!(warn);

#[doc = "Call the browser's `console.clear()` function."]
#[macro_export]
macro_rules! console_clear {
    () => {
        $crate::web_sys::console::clear()
    };
}
