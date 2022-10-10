use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemFn;

/// An attribute macro that performs "Ok-wrapping" on the return value of a `fn` item.
/// This is compatible with [`Result`], [`Option`], [`ControlFlow`], and any type that
/// implements the unstable [`std::ops::Try`] trait.
///
/// [`ControlFlow`]: core::ops::ControlFlow
#[proc_macro_attribute]
pub fn tryvial(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as ItemFn);
    impl_tryvial(item).into()
}

fn impl_tryvial(item: ItemFn) -> TokenStream2 {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item;

    quote! {
        #(#[#attrs])*
        #vis #sig {
            ::core::iter::empty().try_fold(#block, |_, __x: ::core::convert::Infallible| match __x {})
        }
    }
}
