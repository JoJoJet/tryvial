use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use venial::{Declaration, Error, Function};

/// An attribute macro that performs "Ok-wrapping" on the return value of a `fn` item.
/// This is compatible with [`Result`], [`Option`], [`ControlFlow`], and any type that
/// implements the unstable [`std::ops::Try`] trait.
///
/// Using this macro is equivalent to wrapping the body of a fn in a try block.
///
/// Nightly:
/// ```ignore
/// fn fallible_fn(x: T) -> Result<U, E> {
///     try {
///         let a = do_one(x)?;
///         let b = do_two(a)?;
///         b
///     }
/// }
/// ```
///
/// With `try_fn`:
/// ```ignore
/// #[try_fn]
/// fn fallible_fn(x: T) -> Result<U, E> {
///     let a = do_one(x)?;
///     let b = do_two(a)?;
///     b
/// }
/// ```
///
/// [`ControlFlow`]: core::ops::ControlFlow
#[proc_macro_attribute]
pub fn try_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_try_fn(item.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_attribute]
#[deprecated(note = "renamed to `try_fn`")]
pub fn tryvial(_attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_try_fn(item.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn impl_try_fn(input: TokenStream2) -> Result<TokenStream2, Error> {
    let decl = venial::parse_declaration(input)?;
    let Function {
        attributes,
        vis_marker,
        qualifiers,
        tk_fn_keyword,
        name,
        generic_params,
        tk_params_parens: _,
        params,
        where_clause,
        tk_return_arrow: _,
        return_ty,
        tk_semicolon: _,
        body,
    } = match decl {
        Declaration::Function(item) => item,
        _ => Err(Error::new("`#[try_fn]` is supported only on `fn` items"))?,
    };

    let body = body.ok_or(Error::new(
        "`#[try_fn]` can only be used on functions with a body",
    ))?;

    let return_ty = return_ty.map_or_else(|| quote! { () }, |ty| quote! { #ty });

    Ok(quote! {
        #(#attributes)*
        #vis_marker #qualifiers #tk_fn_keyword #name #generic_params ( #params ) -> #return_ty
        #where_clause
        {
            ::core::iter::empty().try_fold(#body, |_, __x: ::core::convert::Infallible| match __x {})
        }
    })
}
