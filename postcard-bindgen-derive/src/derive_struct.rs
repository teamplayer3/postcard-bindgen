use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde_derive_internals::ast::{Field, Style};

pub fn derive_struct(style: Style, ident: Ident, fields: Vec<Field>) -> TokenStream {
    let fields = fields
        .into_iter()
        .filter(|field| !(field.attrs.skip_serializing() || field.attrs.skip_deserializing()))
        .collect::<Vec<_>>();
    let struct_name = ident.to_string();
    derive_struct_style(style, struct_name, fields)
}

fn derive_struct_style<'a>(
    style: Style,
    struct_name: String,
    fields: impl AsRef<[Field<'a>]>,
) -> TokenStream {
    match style {
        Style::Struct => derive_struct_type(struct_name, fields),
        Style::Tuple => derive_tuple_struct_type(struct_name, fields),
        Style::Unit => derive_unit_struct_type(struct_name),
        Style::Newtype => derive_tuple_struct_type(struct_name, fields),
    }
}

fn derive_unit_struct_type(name: String) -> TokenStream {
    quote!(
        let mut ty = _pb::__private::UnitStructType::new(#name.into());
        reg.register_unit_struct_binding(ty);
    )
}

fn derive_tuple_struct_type<'a>(name: String, fields: impl AsRef<[Field<'a>]>) -> TokenStream {
    let body = fields.as_ref().iter().map(|field| {
        let ty = field.ty;
        quote!(ty.register_field::<#ty>())
    });
    quote!(
        let mut ty = _pb::__private::TupleStructType::new(#name.into());
        #(#body);*;
        reg.register_tuple_struct_binding(ty);
    )
}

fn derive_struct_type<'a>(name: String, fields: impl AsRef<[Field<'a>]>) -> TokenStream {
    let body = fields.as_ref().iter().map(|field| {
        let ident_str = field.attrs.name().serialize_name();
        let ty = field.ty;
        quote!(ty.register_field::<#ty>(#ident_str.into()))
    });
    quote!(
        let mut ty = _pb::__private::StructType::new(#name.into(), module_path!());
        #(#body);*;
        reg.register_struct_binding(ty);
    )
}
