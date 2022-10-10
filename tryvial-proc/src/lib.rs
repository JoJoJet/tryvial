use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemFn;

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
