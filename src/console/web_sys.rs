use paste::paste;

#[doc(hidden)]
pub mod __macro {
    pub use js_sys::Array;
    pub use wasm_bindgen::JsValue;
    pub use web_sys::console;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __jsvalue {
    ($value: expr) => {
        ::std::convert::Into::<$crate::__macro::JsValue>::into($value)
    };
}

macro_rules! websys_rules {
    // $dollar is a workaround to allow repetition in nested macros.
    // It needs to be the token `$`.
    ($dollar:tt, $level:ident) => {
        paste! {
            #[doc = "Call the browser's `console." $level "()` function.\n\n\
            The web-sys crate accepts any amount of arguments, all of which must implement `Into<JsValue>`.\n\n\
            See the [wasm-bindgen documentation](../wasm_bindgen/struct.JsValue.html) for more information."]
            #[macro_export]
            macro_rules! [<console_ $level>] {
                () => {
                    $crate::__macro::console::[<$level _0>]()
                };
                ($a:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _1>](&$crate::__jsvalue!($a))
                };
                ($a:expr, $b:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _2>](&$crate::__jsvalue!($a), &$crate::__jsvalue!($b))
                };
                ($a:expr, $b:expr, $c:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _3>](&$crate::__jsvalue!($a), &$crate::__jsvalue!($b), &$crate::__jsvalue!($c))
                };
                ($a:expr, $b:expr, $c:expr, $d:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _4>](
                        &$crate::__jsvalue!($a),
                        &$crate::__jsvalue!($b),
                        &$crate::__jsvalue!($c),
                        &$crate::__jsvalue!($d),
                    )
                };
                ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _5>](
                        &$crate::__jsvalue!($a),
                        &$crate::__jsvalue!($b),
                        &$crate::__jsvalue!($c),
                        &$crate::__jsvalue!($d),
                        &$crate::__jsvalue!($e),
                    )
                };
                ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _6>](
                        &$crate::__jsvalue!($a),
                        &$crate::__jsvalue!($b),
                        &$crate::__jsvalue!($c),
                        &$crate::__jsvalue!($d),
                        &$crate::__jsvalue!($e),
                        &$crate::__jsvalue!($f),
                    )
                };
                ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr $dollar(,)?) => {
                    $crate::__macro::console::[<$level _7>](
                        &$crate::__jsvalue!($a),
                        &$crate::__jsvalue!($b),
                        &$crate::__jsvalue!($c),
                        &$crate::__jsvalue!($d),
                        &$crate::__jsvalue!($e),
                        &$crate::__jsvalue!($f),
                        &$crate::__jsvalue!($g),
                    )
                };
                ($dollar($dollar item:expr),+ $dollar(,)?) => {
                    {
                        let args = ::std::vec![$dollar($crate::__jsvalue!($dollar item)),+];
                        let args = ::std::iter::IntoIterator::into_iter(args).collect::<$crate::__macro::Array>();
                        $crate::__macro::console::$level(&args)
                    }
                };
            }
        }
    }
}

websys_rules!($, debug);
websys_rules!($, dir);
websys_rules!($, dirxml);
websys_rules!($, error);
websys_rules!($, info);
websys_rules!($, log);
websys_rules!($, trace);
websys_rules!($, warn);

#[doc = "Call the browser's `console.clear()` function."]
#[macro_export]
macro_rules! console_clear {
    () => {
        $crate::__macro::console::clear()
    };
}
