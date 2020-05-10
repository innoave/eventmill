#![recursion_limit = "128"]
#![allow(unused_variables)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::export::ToTokens;
use syn::parse_macro_input;
use syn::{Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput};

#[proc_macro_derive(EventType, attributes(event_type, event_type_version, event_source))]
pub fn derive_event_type(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Enum(ref enum_data) => derive_event_type_for_enum(&ast, enum_data),
        Data::Struct(ref struct_data) => derive_event_type_for_struct(&ast, struct_data),
        Data::Union(ref union_data) => derive_event_type_for_union(&ast, union_data),
    }
}

fn derive_event_type_for_enum(ast: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    unimplemented!()
}

fn derive_event_type_for_struct(ast: &DeriveInput, struct_data: &DataStruct) -> TokenStream {
    let (impl_generics, _ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;
    let evversion = find_event_type_version_attribute(&ast.attrs)
        .map(|attr| (&attr.tokens).to_token_stream())
        .unwrap_or_else(|| quote!("V0"));
    let evsource = find_event_source_attribute(&ast.attrs)
        .map(|attr| (&attr.tokens).to_token_stream())
        .unwrap_or_else(|| quote!(""));
    let evtype = find_event_type_attribute(&ast.attrs)
        .map(|attr| (&attr.tokens).to_token_stream())
        .unwrap_or_else(|| quote!(#tname));

    (quote! {
        #[allow(unused_paranthesis)]
        #[allow(unused_qualifications)]
        impl #impl_generics ::eventmill::EventType for #tname #where_clause {
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

fn derive_event_type_for_union(ast: &DeriveInput, union_data: &DataUnion) -> TokenStream {
    panic!("#[derive(EventType)] is only defined for struct and enum types, but not union types")
}

fn find_event_type_attribute<'a>(
    attributes: impl IntoIterator<Item = &'a Attribute>,
) -> Option<&'a Attribute> {
    attributes
        .into_iter()
        .find(|attr| attr.path.segments[0].ident == "event_type")
}

fn find_event_type_version_attribute<'a>(
    attributes: impl IntoIterator<Item = &'a Attribute>,
) -> Option<&'a Attribute> {
    attributes
        .into_iter()
        .find(|attr| attr.path.segments[0].ident == "event_type_version")
}

fn find_event_source_attribute<'a>(
    attributes: impl IntoIterator<Item = &'a Attribute>,
) -> Option<&'a Attribute> {
    attributes
        .into_iter()
        .find(|attr| attr.path.segments[0].ident == "event_source")
}
