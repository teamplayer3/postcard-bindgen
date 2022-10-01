use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde_derive_internals::ast::{Field, Style};

pub fn derive_struct(style: Style, ident: Ident, fields: Vec<Field>) -> TokenStream {
    match style {
        Style::Struct => derive_struct_type(ident, fields),
        Style::Tuple => derive_tuple_struct_type(ident, fields),
        _ => unimplemented!(),
    }
}

fn derive_tuple_struct_type(ident: Ident, fields: Vec<Field>) -> TokenStream {
    let ident_str = ident.to_string();
    let body = fields.iter().map(|field| {
        let ty = field.ty;
        quote!(ty.register_field::<#ty>())
    });
    quote!(
        let mut ty = _pb::TupleStructType::new(#ident_str.into());
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
        let mut ty = _pb::StructType::new(#ident_str.into());
        #(#body);*;
        reg.register_struct_binding(ty);
    )
}
