use crate::helpers::find_attribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::export::ToTokens;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput, Fields};

pub fn derive_event_type_for_enum(ast: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;
    let evversion = find_attribute("event_type_version", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!("V0"));
    let evsource = find_attribute("event_source", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!(""));

    let event_matches = enum_data
        .variants
        .iter()
        .map(|variant| {
            let vname = &variant.ident;
            let default_evtype = &format!("{}::{}", tname, vname);
            let evtype = find_attribute("event_type", &variant.attrs)
                .map(|attr| (&attr.tokens).into_token_stream())
                .unwrap_or_else(|| quote!(#default_evtype));
            match variant.fields {
                Fields::Unit => quote! {
                    #tname::#vname => #evtype,
                },
                Fields::Unnamed(ref fields) => {
                    let field_names = fields
                        .unnamed
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#vname( #(_#field_names,)* ) => #evtype,
                    }
                }
                Fields::Named(ref fields) => {
                    let field_names = fields
                        .named
                        .pairs()
                        .map(|p| p.value().ident.as_ref())
                        .collect::<Vec<_>>();
                    quote! {
                        #tname::#vname { #(#field_names: _,)* } => #evtype,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    (quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::eventmill::EventType for #tname #ty_generics #where_clause {
            fn event_type_version(&self) -> &str {
                #evversion
            }
            fn event_type(&self) -> &str {
                match self {
                    #(#event_matches)*
                }
            }
            fn event_source(&self) -> &str {
                #evsource
            }
        }
    })
    .into()
}

pub fn derive_event_type_for_struct(ast: &DeriveInput, _struct_data: &DataStruct) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;
    let evversion = find_attribute("event_type_version", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!("V0"));
    let evsource = find_attribute("event_source", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!(""));
    let evtype = find_attribute("event_type", &ast.attrs)
        .map(|attr| (&attr.tokens).into_token_stream())
        .unwrap_or_else(|| quote!(stringify!(#tname)));

    (quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::eventmill::EventType for #tname #ty_generics #where_clause {
            fn event_type_version(&self) -> &str {
                #evversion
            }
            fn event_type(&self) -> &str {
                #evtype
            }
            fn event_source(&self) -> &str {
                #evsource
            }
        }
    })
    .into()
}

pub fn derive_event_type_for_union(_ast: &DeriveInput, _union_data: &DataUnion) -> TokenStream {
    panic!("#[derive(EventType)] is only defined for struct and enum types, but not union types")
}
