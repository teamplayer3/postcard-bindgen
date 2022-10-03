pub mod strukt;
pub mod tuple_struct;
pub mod ty_enum;
pub mod unit_struct;

use genco::{
    lang::js::Tokens,
    prelude::JavaScript,
    quote, quote_in,
    tokens::{quoted, FormatInto},
};

use crate::{
    registry::StructField,
    type_info::{JsType, ObjectMeta},
    StringExt,
};

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
                InnerTypeAccess::EnumInner => .inner
            })
        }
    }
}

fn gen_struct_field_available_checks(
    fields: impl AsRef<[StructField]>,
    inner_access: InnerTypeAccess,
) -> Tokens {
    and_chain(
        fields
            .as_ref()
            .iter()
            .map(|field| quote!( $(quoted(&field.name)) in v$inner_access)),
    )
}

fn gen_struct_field_type_checks(
    fields: impl AsRef<[StructField]>,
    inner_access: InnerTypeAccess,
) -> Tokens {
    and_chain(fields.as_ref().iter().map(|field| {
        gen_field_type_check(
            FieldAccess::Object(&field.name),
            &field.js_type,
            inner_access,
        )
    }))
}

fn gen_array_field_type_checks(
    fields: impl AsRef<[JsType]>,
    inner_access: InnerTypeAccess,
) -> Tokens {
    and_chain(
        fields.as_ref().iter().enumerate().map(|(index, field)| {
            gen_field_type_check(FieldAccess::Array(index), field, inner_access)
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
    quote!($(for part in parts join (&&) => $part))
}

fn or_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join (||) => $part))
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
