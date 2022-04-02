use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = ast.ident.clone();
    let b_ident = syn::Ident::new(&format!("{}Builder", ident), ident.span());

    let fields = extract_fields(ast);
    let optionized_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote! {
            #ident: ::std::option::Option<#ty>,
        }
    });
    let methods = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let expanded = quote! {
        impl #ident {
            pub fn builder() -> #b_ident {
                #b_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
        pub struct #b_ident {
            #(#optionized_fields)*
        }
        impl #b_ident {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

fn extract_fields(
    ast: syn::DeriveInput,
) -> syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = ast.data
    {
        fields.named
    } else {
        unimplemented!("Cannot derive builder unless struct")
    }
}
