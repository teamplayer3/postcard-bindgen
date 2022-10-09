mod strukt;
mod tuple_struct;
mod ty_enum;
mod unit_struct;

use genco::{
    lang::js::Tokens,
    prelude::JavaScript,
    quote, quote_in,
    tokens::{quoted, FormatInto},
};

use crate::{
    registry::{BindingType, StructField},
    type_info::{JsType, ObjectMeta},
    utils::StringExt,
};

use super::{line_brake_chain, JS_ENUM_VARIANT_VALUE};

pub fn gen_type_checkings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(|ty| match ty {
        BindingType::Enum(ty) => ty_enum::gen_check_func(&ty.name, &ty.variants),
        BindingType::Struct(ty) => strukt::gen_check_func(&ty.name, &ty.fields),
        BindingType::TupleStruct(ty) => tuple_struct::gen_check_func(&ty.name, &ty.fields),
        BindingType::UnitStruct(ty) => unit_struct::gen_check_func(&ty.name),
    }))
}

#[derive(Debug, Clone, Copy)]
enum FieldAccess<'a> {
    Object(&'a str),
    Array(usize),
}

impl FormatInto<JavaScript> for FieldAccess<'_> {
    fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
        quote_in! { *tokens =>
            $(match self {
                FieldAccess::Array(i) => [$i],
                FieldAccess::Object(n) => .$n
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InnerTypeAccess {
    Direct,
    EnumInner,
}

impl FormatInto<JavaScript> for InnerTypeAccess {
    fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
        quote_in! { *tokens =>
            $(match self {
                InnerTypeAccess::Direct => (),
                InnerTypeAccess::EnumInner => .$JS_ENUM_VARIANT_VALUE
            })
        }
    }
}

fn gen_object_checks(fields: impl AsRef<[StructField]>,
    inner_access: InnerTypeAccess) -> Tokens {
    let field_checks = gen_struct_field_checks(fields, inner_access);
    quote!(typeof v$inner_access === "object" && $field_checks)
}

fn gen_array_checks(fields: impl AsRef<[JsType]>,
    inner_access: InnerTypeAccess) -> Tokens {
    let arr_len = fields.as_ref().len();
    let field_checks = gen_array_field_checks(fields, inner_access);
    quote!(Array.isArray(v$inner_access) && v$inner_access.length === $arr_len && $field_checks)
}

fn gen_struct_field_checks(
    fields: impl AsRef<[StructField]>,
    inner_access: InnerTypeAccess,
) -> Tokens {
    and_chain(fields.as_ref().iter().map(|field| {
        match &field.js_type {
            JsType::Optional(t) => {
                let field_name = &field.name;
                let field_name_str = quoted(field_name);
                let accessor = FieldAccess::Object(field_name);
                let type_check = gen_field_type_check(
                    accessor,
                    t,
                    inner_access,
                );
                quote!((($field_name_str in v$inner_access && (v$inner_access$accessor !== undefined && $type_check) || v$inner_access$accessor === undefined) || !($field_name_str in v$inner_access)))
            },
            _ => {
                let field_name = &field.name;
                let field_name_str = quoted(field_name);
                let type_check = gen_field_type_check(
                    FieldAccess::Object(&field.name),
                    &field.js_type,
                    inner_access,
                );
                quote!($field_name_str in v$inner_access && $type_check)
            }
        }
    }))
}

fn gen_array_field_checks(fields: impl AsRef<[JsType]>, inner_access: InnerTypeAccess) -> Tokens {
    and_chain(
        fields.as_ref().iter().enumerate().map(|(index, field)| {
            match field {
                JsType::Optional(o) => {
                    let accessor = FieldAccess::Array(index);
                    let inner_type_check = gen_field_type_check(accessor, o, inner_access);
                    quote!((v$inner_access$accessor !== undefined && $inner_type_check) || v$inner_access$accessor === undefined)
                }, 
                _ => gen_field_type_check(FieldAccess::Array(index), field, inner_access)
            }            
        }),
    )
}

fn gen_field_type_check(
    field_access: FieldAccess,
    ty: &JsType,
    inner_access: InnerTypeAccess,
) -> Tokens {
    match ty {
        JsType::Array(_) => quote!(Array.isArray(v$inner_access$field_access)),
        JsType::Object(ObjectMeta { name }) => {
            quote!(is_$(name.to_obj_identifier())(v$inner_access$field_access))
        }
        _ => quote!(typeof v$inner_access$field_access === $(quoted(ty.to_string()))),
    }
}

fn and_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join ( && ) => $part))
}

fn or_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join ( || ) => $part))
}

#[cfg(test)]
mod test {
    use genco::quote;

    use super::{and_chain, or_chain};

    #[test]
    fn test_and_chain() {
        let parts = vec![quote!(true === true), quote!(false === false)];
        assert_eq!(
            and_chain(parts).to_string().unwrap(),
            "true === true&&false === false"
        )
    }

    #[test]
    fn test_or_chain() {
        let parts = vec![quote!(true === true), quote!(false === false)];
        assert_eq!(
            or_chain(parts).to_string().unwrap(),
            "true === true||false === false"
        )
    }
}
