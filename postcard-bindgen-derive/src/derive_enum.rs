use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde_derive_internals::ast::{self, Style};

use super::PRIVATE_IMPORT_PATH;

pub fn derive_enum<'a>(ident: Ident, variants: impl AsRef<[ast::Variant<'a>]>) -> TokenStream {
    let enum_name = ident.to_string();
    let body = variants.as_ref().iter().map(|variant| {
        let variant_name = variant.attrs.name().serialize_name();
        derive_variant_style(&variant.style, &variant_name, &variant.fields)
    });
    quote!(
        let mut ty = #PRIVATE_IMPORT_PATH::EnumType::new(#enum_name.into());
        #(#body);*;
        reg.register_enum_binding(ty);
    )
}

fn derive_variant_style<'a>(
    style: &Style,
    variant_name: impl AsRef<str>,
    fields: impl AsRef<[ast::Field<'a>]>,
) -> TokenStream {
    match style {
        ast::Style::Struct => derive_struct_variant(variant_name, fields),
        ast::Style::Newtype => derive_newtype_variant(variant_name, &fields.as_ref()[0]),
        ast::Style::Tuple => derive_tuple_variant(variant_name, fields),
        ast::Style::Unit => derive_unit_variant(variant_name),
    }
}

fn derive_unit_variant(variant_name: impl AsRef<str>) -> TokenStream {
    let variant_name = variant_name.as_ref();
    quote!(ty.register_variant(#variant_name.into());)
}

fn derive_newtype_variant(variant_name: impl AsRef<str>, field: &ast::Field<'_>) -> TokenStream {
    let variant_name = variant_name.as_ref();
    let ty = field.ty;
    quote!(
        let mut fields = #PRIVATE_IMPORT_PATH::TupleFields::default();
        fields.register_field::<#ty>();
        ty.register_variant_tuple(#variant_name.into(), fields);
    )
}

fn derive_struct_variant<'a>(
    variant_name: impl AsRef<str>,
    fields: impl AsRef<[ast::Field<'a>]>,
) -> TokenStream {
    let variant_name = variant_name.as_ref();
    let body = fields.as_ref().iter().map(|field| {
        let ty = field.ty;
        let field_name = field.attrs.name().serialize_name();
        quote!(fields.register_field::<#ty>(#field_name.into());)
    });
    quote!(
        let mut fields = #PRIVATE_IMPORT_PATH::StructFields::default();
        #(#body);*;
        ty.register_unnamed_struct(#variant_name.into(), fields);
    )
}

fn derive_tuple_variant<'a>(
    variant_name: impl AsRef<str>,
    fields: impl AsRef<[ast::Field<'a>]>,
) -> TokenStream {
    let variant_name = variant_name.as_ref();
    let body = fields.as_ref().iter().map(|field| {
        let ty = field.ty;
        quote!(fields.register_field::<#ty>();)
    });
    quote!(
        let mut fields = #PRIVATE_IMPORT_PATH::TupleFields::default();
        #(#body);*;
        ty.register_variant_tuple(#variant_name.into(), fields);
    )
}
