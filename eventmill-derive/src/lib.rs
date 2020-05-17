#![recursion_limit = "128"]

extern crate proc_macro;

mod aggregate_type;
mod event_type;
mod helpers;

use crate::aggregate_type::{
    derive_aggregate_type_for_enum, derive_aggregate_type_for_struct,
    derive_aggregate_type_for_union,
};
use crate::event_type::{
    derive_event_type_for_enum, derive_event_type_for_struct, derive_event_type_for_union,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

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

#[proc_macro_derive(AggregateType, attributes(aggregate_type, id_field))]
pub fn derive_aggregate_type(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", ast);

    match ast.data {
        Data::Enum(ref enum_data) => derive_aggregate_type_for_enum(&ast, enum_data),
        Data::Struct(ref struct_data) => derive_aggregate_type_for_struct(&ast, struct_data),
        Data::Union(ref union_data) => derive_aggregate_type_for_union(&ast, union_data),
    }
}
