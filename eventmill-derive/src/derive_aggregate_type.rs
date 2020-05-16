use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput};

pub fn derive_aggregate_type_for_struct(
    _ast: &DeriveInput,
    _struct_data: &DataStruct,
) -> TokenStream {
    (quote! {}).into()
}

pub fn derive_aggregate_type_for_enum(_ast: &DeriveInput, _enum_data: &DataEnum) -> TokenStream {
    panic!("#[derive(AggregateType)] is only defined for struct types, but not enum or union types")
}

pub fn derive_aggregate_type_for_union(_ast: &DeriveInput, _union_data: &DataUnion) -> TokenStream {
    panic!("#[derive(AggregateType)] is only defined for struct types, but not enum or union types")
}
