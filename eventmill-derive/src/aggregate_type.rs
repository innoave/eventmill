use crate::helpers::{find_attribute, find_struct_field};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput};

pub fn derive_aggregate_type_for_struct(
    ast: &DeriveInput,
    struct_data: &DataStruct,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let tname = &ast.ident;
    let aggtype = find_attribute("aggregate_type", &ast.attrs)
        .map(|attr| attr.tokens.clone())
        .unwrap_or_else(|| quote!(stringify!(#tname)));

    let mut output = quote! {
        #[allow(unused_qualifications, unused_parens)]
        #[automatically_derived]
        impl #impl_generics ::eventmill::AggregateType for #tname #ty_generics #where_clause {
            fn aggregate_type() -> &'static str {
                #aggtype
            }
        }
    };

    if let Some(id_field_name) = find_attribute("id_field", &ast.attrs)
        .and_then(|attr| Some(attr.parse_args::<Ident>().expect("field identifier")))
    {
        let maybe_id_field = find_struct_field(&id_field_name.to_string(), &struct_data.fields);

        if let Some(id_field) = maybe_id_field {
            let idfname = id_field.ident.as_ref().unwrap();
            let idftype = &id_field.ty;

            output.extend(quote! {
                #[allow(unused_qualifications, unused_parens)]
                #[automatically_derived]
                impl #impl_generics ::eventmill::WithAggregateId for #tname #ty_generics #where_clause {
                    type Id = #idftype;
                    fn aggregate_id(&self) -> &Self::Id {
                        &self.#idfname
                    }
                }
            })
        } else {
            output.extend(
                syn::Error::new(
                    id_field_name.span(),
                    format!("no field {} on struct {}", id_field_name, tname),
                )
                .to_compile_error(),
            );
        }
    };

    output.into()
}

pub fn derive_aggregate_type_for_enum(_ast: &DeriveInput, _enum_data: &DataEnum) -> TokenStream {
    panic!("#[derive(AggregateType)] is only defined for struct types, but not enum or union types")
}

pub fn derive_aggregate_type_for_union(_ast: &DeriveInput, _union_data: &DataUnion) -> TokenStream {
    panic!("#[derive(AggregateType)] is only defined for struct types, but not enum or union types")
}
