#[doc(hidden)]
pub use ::wasm_bindgen;
#[doc(hidden)]
pub use ::web_sys;

// It is necessary to expose these wrapper macros in order to pass the $crate
// meta-variable to the proc macro. This in turn allows the macros to be
// re-exported from third-party crates without breakage.
//
// TODO: a declarative macro to remove the duplication here would be nice.

#[doc(hidden)]
pub use ::weblog_proc_macro::console_assert as __weblog_console_assert;

#[macro_export]
/// Call the browser's `console.assert()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/assert)
macro_rules! console_assert {
    ($($input:tt)*) => {
        $crate::__weblog_console_assert! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_clear as __weblog_console_clear;

#[macro_export]
/// Call the browser's `console.clear()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
macro_rules! console_clear {
    ($($input:tt)*) => {
        $crate::__weblog_console_clear! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_count as __weblog_console_count;

#[macro_export]
/// Call the browser's `console.count()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/count)
macro_rules! console_count {
    ($($input:tt)*) => {
        $crate::__weblog_console_count! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_count_reset as __weblog_console_count_reset;

#[macro_export]
/// Call the browser's `console.count_reset()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/count_reset)
macro_rules! console_count_reset {
    ($($input:tt)*) => {
        $crate::__weblog_console_count_reset! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_debug as __weblog_console_debug;

#[macro_export]
/// Call the browser's `console.debug()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
macro_rules! console_debug {
    ($($input:tt)*) => {
        $crate::__weblog_console_debug! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_dir as __weblog_console_dir;

#[macro_export]
/// Call the browser's `console.dir()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/dir)
macro_rules! console_dir {
    ($($input:tt)*) => {
        $crate::__weblog_console_dir! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_dirxml as __weblog_console_dirxml;

#[macro_export]
/// Call the browser's `console.dirxml()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/dirxml)
macro_rules! console_dirxml {
    ($($input:tt)*) => {
        $crate::__weblog_console_dirxml! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_error as __weblog_console_error;

#[macro_export]
/// Call the browser's `console.error()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
macro_rules! console_error {
    ($($input:tt)*) => {
        $crate::__weblog_console_error! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_exception as __weblog_console_exception;

#[macro_export]
/// Call the browser's `console.exception()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/exception)
macro_rules! console_exception {
    ($($input:tt)*) => {
        $crate::__weblog_console_exception! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_info as __weblog_console_info;

#[macro_export]
/// Call the browser's `console.info()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
macro_rules! console_info {
    ($($input:tt)*) => {
        $crate::__weblog_console_info! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_log as __weblog_console_log;

#[macro_export]
/// Call the browser's `console.log()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
macro_rules! console_log {
    ($($input:tt)*) => {
        $crate::__weblog_console_log! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_table as __weblog_console_table;

#[macro_export]
/// Call the browser's `console.table()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/table)
macro_rules! console_table {
    ($($input:tt)*) => {
        $crate::__weblog_console_table! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_time as __weblog_console_time;

#[macro_export]
/// Call the browser's `console.time()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/time)
macro_rules! console_time {
    ($($input:tt)*) => {
        $crate::__weblog_console_time! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_time_end as __weblog_console_time_end;

#[macro_export]
/// Call the browser's `console.time_end()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_end)
macro_rules! console_time_end {
    ($($input:tt)*) => {
        $crate::__weblog_console_time_end! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_time_log as __weblog_console_time_log;

#[macro_export]
/// Call the browser's `console.time_log()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_log)
macro_rules! console_time_log {
    ($($input:tt)*) => {
        $crate::__weblog_console_time_log! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_time_stamp as __weblog_console_time_stamp;

#[macro_export]
/// Call the browser's `console.time_stamp()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_stamp)
macro_rules! console_time_stamp {
    ($($input:tt)*) => {
        $crate::__weblog_console_time_stamp! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_trace as __weblog_console_trace;

#[macro_export]
/// Call the browser's `console.trace()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/trace)
macro_rules! console_trace {
    ($($input:tt)*) => {
        $crate::__weblog_console_trace! {
            #![crate = $crate]
            $($input)*
        }
    };
}

#[doc(hidden)]
pub use ::weblog_proc_macro::console_warn as __weblog_console_warn;

#[macro_export]
/// Call the browser's `console.warn()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/warn)
macro_rules! console_warn {
    ($($input:tt)*) => {
        $crate::__weblog_console_warn! {
            #![crate = $crate]
            $($input)*
        }
    };
}
