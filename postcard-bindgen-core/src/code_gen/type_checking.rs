use convert_case::{Case, Casing};
use genco::{quote, tokens::quoted, Tokens};

use crate::{
    registry::StructField,
    type_info::{JsType, ObjectMeta},
};

pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
    let obj_name = obj_name.as_ref();

    quote! {
        const is_$(obj_name.to_case(Case::Snake).to_uppercase()) = (v) => {
            return $(gen_field_checks(&fields).iter().chain(&gen_type_checks(fields)).map(|q| q.to_string().unwrap()).collect::<Vec<_>>().join("&&"))
        }
    }
}

fn gen_field_checks(fields: impl AsRef<[StructField]>) -> Vec<Tokens> {
    fields
        .as_ref()
        .iter()
        .map(|field| quote!( $(quoted(&field.name)) in v))
        .collect::<Vec<_>>()
}

fn gen_type_checks(fields: impl AsRef<[StructField]>) -> Vec<Tokens> {
    fields
        .as_ref()
        .iter()
        .map(gen_type_check)
        .collect::<Vec<_>>()
}

fn gen_type_check(field: &StructField) -> Tokens {
    match &field.js_type {
        JsType::Array(_) => quote!(Array.isArray(v.$(&field.name))),
        JsType::Object(ObjectMeta { name }) => {
            quote!(is_$(name.to_case(Case::Snake).to_uppercase())(v.$(&field.name)))
        }
        _ => quote!(typeof v.$(field.name.as_str()) === $(quoted(field.js_type.to_string()))),
    }
}
