pub extern crate stdweb;
pub use stdweb::js;

#[doc = "Call the browser's `console.assert()` function."]
#[macro_export]
macro_rules! console_assert {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.assert($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.clear()` function."]
#[macro_export]
macro_rules! console_clear {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.clear() }
    }};
}

#[doc = "Call the browser's `console.count()` function."]
#[macro_export]
macro_rules! console_count {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.count($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.countReset()` function."]
#[macro_export]
macro_rules! console_count_reset {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.countReset($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.debug()` function."]
#[macro_export]
macro_rules! console_debug {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.debug($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.dir()` function."]
#[macro_export]
macro_rules! console_dir {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.dir($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.dirxml()` function."]
#[macro_export]
macro_rules! console_dirxml {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.dirxml($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.error()` function."]
#[macro_export]
macro_rules! console_error {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.error($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.exception()` function."]
#[macro_export]
macro_rules! console_exception {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.exception($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.info()` function."]
#[macro_export]
macro_rules! console_info {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.info($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.log()` function."]
#[macro_export]
macro_rules! console_log {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.log($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.table()` function."]
#[macro_export]
macro_rules! console_table {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.table($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.time()` function."]
#[macro_export]
macro_rules! console_time {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.time($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.timeEnd()` function."]
#[macro_export]
macro_rules! console_time_end {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.timeEnd($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.timeLog()` function."]
#[macro_export]
macro_rules! console_time_log {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.timeLog($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.timeStamp()` function."]
#[macro_export]
macro_rules! console_time_stamp {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.timeStamp($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.trace()` function."]
#[macro_export]
macro_rules! console_trace {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.trace($( @{$item} ),*) }
    }};
}

#[doc = "Call the browser's `console.warn()` function."]
#[macro_export]
macro_rules! console_warn {
    ($($item:expr),* $(,)?) => {{
        $crate::stdweb::js! { console.warn($( @{$item} ),*) }
    }};
}
