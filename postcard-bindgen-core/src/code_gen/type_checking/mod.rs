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

use super::JS_OBJECT_VARIABLE;

pub fn gen_type_checkings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(gen_type_check))
}

pub fn gen_type_check(binding_type: &BindingType) -> Tokens {
    let type_name = binding_type.inner_name().to_obj_identifier();
    let body = match binding_type {
        BindingType::Enum(ty) => ty_enum::gen_check_func(&ty.variants),
        BindingType::Struct(ty) => gen_object_checks(&ty.fields, ser::VariablePath::default()),
        BindingType::TupleStruct(ty) => gen_array_checks(&ty.fields, ser::VariablePath::default()),
        BindingType::UnitStruct(_) => unit_struct::gen_check_func(),
    };
    quote!(const is_$type_name = ($JS_OBJECT_VARIABLE) => ($body))
}

fn gen_object_checks(
    fields: impl AsRef<[StructField]>,
    variable_path: ser::VariablePath,
) -> Tokens {
    let field_checks = gen_struct_field_checks(fields, variable_path.to_owned());
    quote!(typeof $variable_path === "object" && $field_checks)
}

fn gen_array_checks(fields: impl AsRef<[JsType]>, variable_path: ser::VariablePath) -> Tokens {
    let arr_len = fields.as_ref().len();
    let field_checks = gen_array_field_checks(fields, variable_path.to_owned());
    quote!(Array.isArray($(variable_path.to_owned())) && $variable_path.length === $arr_len && $field_checks)
}

fn gen_struct_field_checks(
    fields: impl AsRef<[StructField]>,
    variable_path: ser::VariablePath,
) -> Tokens {
    and_chain(fields.as_ref().iter().map(|field| {
        let path = variable_path
            .to_owned()
            .modify_push(ser::VariableAccess::Field(field.name.into()));
        field.js_type.gen_ty_check(path)
    }))
}

fn gen_array_field_checks(
    fields: impl AsRef<[JsType]>,
    variable_path: ser::VariablePath,
) -> Tokens {
    and_chain(fields.as_ref().iter().enumerate().map(|(index, field)| {
        let path = variable_path
            .to_owned()
            .modify_push(ser::VariableAccess::Indexed(index));
        field.gen_ty_check(path)
    }))
}
