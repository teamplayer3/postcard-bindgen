use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde_derive_internals::ast::{Field, Style};

use super::PRIVATE_IMPORT_PATH;

pub fn derive_struct(style: Style, ident: Ident, fields: Vec<Field>) -> TokenStream {
    let fields = fields
        .into_iter()
        .filter(|field| !(field.attrs.skip_serializing() || field.attrs.skip_deserializing()))
        .collect::<Vec<_>>();
    match style {
        Style::Struct => derive_struct_type(ident, fields),
        Style::Tuple => derive_tuple_struct_type(ident, fields),
        Style::Unit => derive_unit_struct_type(ident),
        Style::Newtype => derive_tuple_struct_type(ident, fields),
    }
}

fn derive_unit_struct_type(ident: Ident) -> TokenStream {
    let ident_str = ident.to_string();
    quote!(
        let mut ty = #PRIVATE_IMPORT_PATH::UnitStructType::new(#ident_str.into());
        reg.register_unit_struct_binding(ty);
    )
}

fn derive_tuple_struct_type(ident: Ident, fields: Vec<Field>) -> TokenStream {
    let ident_str = ident.to_string();
    let body = fields.iter().map(|field| {
        let ty = field.ty;
        quote!(ty.register_field::<#ty>())
    });
    quote!(
        let mut ty = #PRIVATE_IMPORT_PATH::TupleStructType::new(#ident_str.into());
        #(#body);*;
        reg.register_tuple_struct_binding(ty);
    )
}

fn derive_struct_type(ident: Ident, fields: Vec<Field>) -> TokenStream {
    let ident_str = ident.to_string();
    let body = fields.iter().map(|field| {
        let ident_str = field.attrs.name().serialize_name();
        let ty = field.ty;
        quote!(ty.register_field::<#ty>(#ident_str.into()))
    });
    quote!(
        let mut ty = #PRIVATE_IMPORT_PATH::StructType::new(#ident_str.into());
        #(#body);*;
        reg.register_struct_binding(ty);
    )
}
