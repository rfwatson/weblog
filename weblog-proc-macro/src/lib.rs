extern crate proc_macro2;

mod weblog_impl;

use proc_macro::TokenStream;
use std::iter;
use weblog_impl::{quote_console_func, ArgMode, ConsoleFunc};

/// Call the browser's `console.assert()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/assert)
#[proc_macro]
pub fn console_assert(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("assert_with_condition_and_data", 1),
        iter::once(ArgMode::PassThrough).chain(iter::repeat(ArgMode::IntoJsValue)),
        input,
    )
}

/// Call the browser's `console.clear()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
#[proc_macro]
pub fn console_clear(input: TokenStream) -> TokenStream {
    quote_console_func(ConsoleFunc::fixed("clear", 0), iter::empty(), input)
}

/// Call the browser's `console.count()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/count)
#[proc_macro]
pub fn console_count(input: TokenStream) -> TokenStream {
    let name = if input.is_empty() {
        "count"
    } else {
        "count_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        input,
    )
}

/// Call the browser's `console.countReset()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/countReset)
#[proc_macro]
pub fn console_count_reset(input: TokenStream) -> TokenStream {
    let name = if input.is_empty() {
        "count_reset"
    } else {
        "count_reset_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        input,
    )
}

/// Call the browser's `console.debug()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
#[proc_macro]
pub fn console_debug(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("debug", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.dir()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/dir)
#[proc_macro]
pub fn console_dir(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("dir", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.dirxml()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/dirxml)
#[proc_macro]
pub fn console_dirxml(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("dirxml", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.error()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
#[proc_macro]
pub fn console_error(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("error", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.exception()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/exception)
#[proc_macro]
pub fn console_exception(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("exception", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.info()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
#[proc_macro]
pub fn console_info(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("info", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.log()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
#[proc_macro]
pub fn console_log(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("log", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.table()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/table)
#[proc_macro]
pub fn console_table(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("table", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.time()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/time)
#[proc_macro]
pub fn console_time(input: TokenStream) -> TokenStream {
    let name = if input.is_empty() {
        "time"
    } else {
        "time_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        input,
    )
}

/// Call the browser's `console.timeEnd()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/timeEnd)
#[proc_macro]
pub fn console_time_end(input: TokenStream) -> TokenStream {
    let name = if input.is_empty() {
        "time_end"
    } else {
        "time_end_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        input,
    )
}

/// Call the browser's `console.timeLog()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/timeLog)
#[proc_macro]
pub fn console_time_log(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("time_log_with_label_and_data", 1),
        iter::once(ArgMode::PassThrough).chain(iter::repeat(ArgMode::IntoJsValue)),
        input,
    )
}

/// Call the browser's `console.timeStamp()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/timeStamp)
#[proc_macro]
pub fn console_time_stamp(input: TokenStream) -> TokenStream {
    let name = if input.is_empty() {
        "time_stamp"
    } else {
        "time_stamp_with_data"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.trace()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/trace)
#[proc_macro]
pub fn console_trace(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("trace", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}

/// Call the browser's `console.warn()` function.
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console/warn)
#[proc_macro]
pub fn console_warn(input: TokenStream) -> TokenStream {
    quote_console_func(
        ConsoleFunc::variadic("warn", 0),
        iter::repeat(ArgMode::IntoJsValue),
        input,
    )
}
