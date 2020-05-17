use proc_macro2::Ident;
use syn::{Attribute, Field, Fields, FieldsNamed};

pub fn find_attribute<'a>(
    name: &str,
    attributes: impl IntoIterator<Item = &'a Attribute>,
) -> Option<&'a Attribute> {
    attributes
        .into_iter()
        .find(|attr| attr.path.segments[0].ident == name)
}

pub fn find_struct_field<'a>(name: &str, fields: &'a Fields) -> Option<&'a Field> {
    match fields {
        Fields::Named(named_fields) => find_named_field(name, named_fields),
        Fields::Unnamed(_) => None,
        Fields::Unit => None,
    }
}

fn find_named_field<'a>(name: &str, fields: &'a FieldsNamed) -> Option<&'a Field> {
    fields.named.iter().find(|fld| {
        if let Some(ident) = &fld.ident {
            ident == name
        } else {
            false
        }
    })
}

pub fn list_field_idents(fields: &FieldsNamed) -> impl Iterator<Item = &Ident> {
    fields.named.iter().filter_map(|fld| fld.ident.as_ref())
}
