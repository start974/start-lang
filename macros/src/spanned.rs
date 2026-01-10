use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};
use quote::quote;

pub fn derive_spanned(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data) => derive_spanned_struct(name, data),
        Data::Enum(data) => derive_spanned_enum(name, data),
        Data::Union(_) => panic!("Spanned does not support unions"),
    };

    TokenStream::from(expanded)
}

fn derive_spanned_struct(name: &syn::Ident, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let _ = data
        .fields
        .iter()
        .find(|f| f.ident.as_ref().map(|id| id == "span").unwrap_or(false))
        .expect("Spanned requires a `span: Span` field");

    quote! {
        impl Spanned for #name {
            fn span(&self) -> Span {
                self.span
            }
        }
    }
}

fn derive_spanned_enum(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let arms = data.variants.iter().map(|v| {
        let vname = &v.ident;

        quote! {
            #name::#vname { span, .. } => *span,
            #name::#vname(x) => x.span(),
        }
    });

    quote! {
        impl Spanned for #name {
            fn span(&self) -> Span {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}
