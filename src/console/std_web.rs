pub extern crate stdweb;
pub use stdweb::js;

#[doc = "Call the browser's `console.debug()` function."]
#[macro_export]
macro_rules! console_debug {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.debug($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.clear()` function."]
#[macro_export]
macro_rules! console_clear {
    ($item:expr) => {{
        $crate::stdweb::js! { console.clear() }
    }};
}

#[doc = "Call the browser's `console.dir()` function."]
#[macro_export]
macro_rules! console_dir {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.dir($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.dirxml()` function."]
#[macro_export]
macro_rules! console_dirxml {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.dirxml($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.error()` function."]
#[macro_export]
macro_rules! console_error {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.error($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.info()` function."]
#[macro_export]
macro_rules! console_info {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.info($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.log()` function."]
#[macro_export]
macro_rules! console_log {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.log($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.trace()` function."]
#[macro_export]
macro_rules! console_trace {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.trace($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.warn()` function."]
#[macro_export]
macro_rules! console_warn {
    ($( $item:expr ),* ) => {{
        $crate::stdweb::js! { console.warn($( @{$item} ),*) }
    }};
}
