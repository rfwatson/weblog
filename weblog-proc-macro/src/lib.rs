//! weblog-proc-macro contains procedural macro definitions for the
//! [`weblog`](https://crates.io/crates/weblog) crate. See its documentation for more information.
extern crate proc_macro2;

mod weblog_impl;

use proc_macro::TokenStream;
use std::iter;
use syn::parse_macro_input;
use weblog_impl::{quote_console_func, ArgMode, ConsoleFunc, InputTokens};

#[doc(hidden)]
#[proc_macro]
pub fn console_assert(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("assert_with_condition_and_data", 1),
        iter::once(ArgMode::PassThrough).chain(iter::repeat(ArgMode::IntoJsValue)),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_clear(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::fixed("clear", 0),
        iter::empty(),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_count(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    let name = if args.is_empty() {
        "count"
    } else {
        "count_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_count_reset(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    let name = if args.is_empty() {
        "count_reset"
    } else {
        "count_reset_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_debug(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("debug", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_dir(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("dir", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_dirxml(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("dirxml", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_error(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("error", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_exception(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("exception", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_info(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("info", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_log(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("log", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_table(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("table", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_time(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    let name = if args.is_empty() {
        "time"
    } else {
        "time_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_time_end(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    let name = if args.is_empty() {
        "time_end"
    } else {
        "time_end_with_label"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::PassThrough),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_time_log(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("time_log_with_label_and_data", 1),
        iter::once(ArgMode::PassThrough).chain(iter::repeat(ArgMode::IntoJsValue)),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_time_stamp(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    let name = if args.is_empty() {
        "time_stamp"
    } else {
        "time_stamp_with_data"
    };

    quote_console_func(
        ConsoleFunc::fixed(name, 1),
        iter::once(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_trace(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("trace", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}

#[doc(hidden)]
#[proc_macro]
pub fn console_warn(input: TokenStream) -> TokenStream {
    let InputTokens { crate_name, args } = parse_macro_input!(input as InputTokens);

    quote_console_func(
        ConsoleFunc::variadic("warn", 0),
        iter::repeat(ArgMode::IntoJsValue),
        crate_name,
        args,
    )
}
