pub mod ts;
mod ty_enum;
mod unit_struct;

use genco::{lang::js::Tokens, quote};

use crate::{
    code_gen::{
        generateable::js_types::*,
        utils::{and_chain, line_brake_chain},
    },
    registry::{BindingType, StructField},
    type_info::JsType,
    utils::StrExt,
};

pub fn gen_type_checkings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(gen_type_check))
}

pub fn gen_type_check(binding_type: &BindingType) -> Tokens {
    let type_name = binding_type.inner_name().to_obj_identifier();
    let body = match binding_type {
        BindingType::Enum(ty) => ty_enum::gen_check_func(&ty.variants),
        BindingType::Struct(ty) => gen_object_checks(&ty.fields, ty_check::InnerTypeAccess::Direct),
        BindingType::TupleStruct(ty) => {
            gen_array_checks(&ty.fields, ty_check::InnerTypeAccess::Direct)
        }
        BindingType::UnitStruct(_) => unit_struct::gen_check_func(),
    };
    quote!(const is_$type_name = (v) => ($body))
}

fn gen_object_checks(
    fields: impl AsRef<[StructField]>,
    inner_access: ty_check::InnerTypeAccess,
) -> Tokens {
    let field_checks = gen_struct_field_checks(fields, inner_access);
    quote!(typeof v$inner_access === "object" && $field_checks)
}

fn gen_array_checks(
    fields: impl AsRef<[JsType]>,
    inner_access: ty_check::InnerTypeAccess,
) -> Tokens {
    let arr_len = fields.as_ref().len();
    let field_checks = gen_array_field_checks(fields, inner_access);
    quote!(Array.isArray(v$inner_access) && v$inner_access.length === $arr_len && $field_checks)
}

fn gen_struct_field_checks(
    fields: impl AsRef<[StructField]>,
    inner_access: ty_check::InnerTypeAccess,
) -> Tokens {
    and_chain(fields.as_ref().iter().map(|field| {
        field
            .js_type
            .gen_ty_check(ty_check::FieldAccess::Object(field.name), inner_access)
    }))
}

fn gen_array_field_checks(
    fields: impl AsRef<[JsType]>,
    inner_access: ty_check::InnerTypeAccess,
) -> Tokens {
    and_chain(fields.as_ref().iter().enumerate().map(|(index, field)| {
        field.gen_ty_check(ty_check::FieldAccess::Array(index), inner_access)
    }))
}
