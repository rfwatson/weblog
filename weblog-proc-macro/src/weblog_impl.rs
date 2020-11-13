use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::cmp;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, Expr, Result, Token};

// The maximum number of variadic arguments accepted by web-sys function
// groups such as `console::log_n`:
const MAX_VARIADIC_ARGS: usize = 7;

// quote_console_func builds and quotes a call to the browser's console API, based on the
// provided console function and argument types and the input token stream provider by
// the macro caller.
//
// Arguments:
//
// * `func`: an object containing the target function name and parameter configuration.
// * `arg_modes`: an Iterator that will be called once per argument parsed from the input tokens.
// * `crate_name`: an identifier that resolves to the name of the calling crate.
// * `in_args`: zero or more punctuated expressions passed as arguments by the caller.
pub fn quote_console_func(
    ConsoleFunc {
        name,
        params_fixed,
        is_variadic,
    }: ConsoleFunc,
    mut arg_modes: impl Iterator<Item = ArgMode>,
    crate_name: Ident,
    in_args: Punctuated<Expr, Token![,]>,
) -> TokenStream {
    let mut out_args = in_args
        .iter()
        .map(|arg| quote_arg(&crate_name, arg, arg_modes.next().unwrap_or_default()));

    let num_provided = out_args.len();
    let num_fixed = cmp::min(num_provided, params_fixed);
    let num_variadic = cmp::max(num_provided - num_fixed, 0);

    let ident = {
        let func_name = if is_variadic && num_variadic <= MAX_VARIADIC_ARGS {
            format!("{}_{}", name, num_variadic)
        } else {
            name
        };
        Ident::new(&func_name, Span::call_site())
    };

    let mut args: Punctuated<_, Token![,]> = Punctuated::new();
    {
        for _ in 0..num_fixed {
            args.push(out_args.next().unwrap());
        }
        if num_variadic > MAX_VARIADIC_ARGS {
            let variadic_args = out_args.collect::<Punctuated<_, Token![,]>>();
            let ary = quote! {
                &::std::iter::FromIterator::from_iter(::std::iter::IntoIterator::into_iter(::std::vec![#variadic_args]))
            };
            args.push(ary);
        } else {
            for _ in 0..num_variadic {
                args.push(out_args.next().unwrap());
            }
        }
    }

    (quote! { #crate_name::web_sys::console::#ident(#args) }).into()
}

fn quote_arg(crate_name: &Ident, arg: &Expr, arg_mode: ArgMode) -> TokenStream2 {
    match arg_mode {
        ArgMode::PassThrough => {
            quote! { #arg }
        }
        ArgMode::IntoJsValue => {
            quote! {
                &::std::convert::Into::<#crate_name::wasm_bindgen::JsValue>::into(#arg)
            }
        }
    }
}

// ConsoleFunc represents the console API function being targeted.
#[derive(Debug)]
pub struct ConsoleFunc {
    name: String,
    params_fixed: usize,
    is_variadic: bool,
}

impl ConsoleFunc {
    pub fn fixed(name: &str, params_fixed: usize) -> Self {
        Self {
            name: name.into(),
            params_fixed,
            is_variadic: false,
        }
    }

    pub fn variadic(name: &str, params_fixed: usize) -> Self {
        Self {
            name: name.into(),
            params_fixed,
            is_variadic: true,
        }
    }
}

// ArgMode represents an argument being passed to a targeted ConsoleFunc.
#[derive(Debug, Copy, Clone)]
pub enum ArgMode {
    PassThrough,
    IntoJsValue,
}

impl Default for ArgMode {
    fn default() -> Self {
        Self::PassThrough
    }
}

pub struct InputTokens {
    pub crate_name: Ident,
    pub args: Punctuated<Expr, Token![,]>,
}

impl Parse for InputTokens {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![#]>()?;
        input.parse::<Token![!]>()?;

        let bracketed;
        bracketed!(bracketed in input);
        bracketed.parse::<Token![crate]>()?;
        bracketed.parse::<Token![ = ]>()?;

        let crate_name = bracketed.parse::<Ident>()?;
        let args: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(&input)?;

        Ok(Self { crate_name, args })
    }
}
