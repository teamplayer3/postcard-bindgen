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

fn gen_struct_field_available_checks(fields: impl AsRef<[StructField]>) -> Tokens {
    and_chain(
        fields
            .as_ref()
            .iter()
            .map(|field| quote!( $(quoted(&field.name)) in v)),
    )
}

fn gen_struct_field_type_checks(fields: impl AsRef<[StructField]>) -> Tokens {
    and_chain(
        fields
            .as_ref()
            .iter()
            .map(|field| gen_field_type_check(FieldAccess::Object(&field.name), &field.js_type)),
    )
}

fn gen_array_field_type_checks(fields: impl AsRef<[JsType]>) -> Tokens {
    and_chain(
        fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(index, field)| gen_field_type_check(FieldAccess::Array(index), field)),
    )
}

fn gen_field_type_check(field_access: FieldAccess, ty: &JsType) -> Tokens {
    match ty {
        JsType::Array(_) => quote!(Array.isArray(v$field_access)),
        JsType::Object(ObjectMeta { name }) => {
            quote!(is_$(name.to_obj_identifier())(v$field_access))
        }
        _ => quote!(typeof v$field_access === $(quoted(ty.to_string()))),
    }
}

pub mod strukt {
    use genco::{lang::js::Tokens, quote};

    use crate::{registry::StructField, StrExt};

    use super::{gen_struct_field_available_checks, gen_struct_field_type_checks};

    pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
        let obj_name = obj_name.as_ref();

        let field_available_checks = gen_struct_field_available_checks(&fields);
        let field_type_checks = gen_struct_field_type_checks(&fields);

        quote! {
            const is_$(obj_name.to_obj_identifier()) = (v) => {
                return typeof v === "object" && $field_available_checks && $field_type_checks
            }
        }
    }
}

pub mod tuple_struct {
    use genco::{lang::js::Tokens, quote};

    use crate::{type_info::JsType, StrExt};

    use super::gen_array_field_type_checks;

    pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[JsType]>) -> Tokens {
        let obj_name = obj_name.as_ref();

        let field_count = fields.as_ref().len();
        let field_type_checks = gen_array_field_type_checks(fields);

        quote! {
            const is_$(obj_name.to_obj_identifier()) = (v) => {
                return Array.isArray(v) && v.length === $field_count && $field_type_checks
            }
        }
    }
}

pub mod unit_struct {
    use genco::{lang::js::Tokens, quote};

    use crate::StrExt;

    pub fn gen_check_func(obj_name: impl AsRef<str>) -> Tokens {
        let obj_name = obj_name.as_ref();

        quote! {
            const is_$(obj_name.to_obj_identifier()) = (v) => {
                return typeof v === "object" && Object.keys(v).length === 0
            }
        }
    }
}

pub mod enum_ty {
    use genco::{lang::js::Tokens, quote, tokens::quoted};

    use crate::{
        registry::{EnumVariant, EnumVariantType},
        StrExt,
    };

    use super::{
        and_chain, gen_array_field_type_checks, gen_struct_field_available_checks,
        gen_struct_field_type_checks, or_chain,
    };

    pub fn gen_check_func(
        obj_name: impl AsRef<str>,
        variants: impl AsRef<[EnumVariant]>,
    ) -> Tokens {
        let obj_name = obj_name.as_ref();
        let enumerated_variants = variants.as_ref().iter().enumerate();
        let simple_variants = enumerated_variants
            .to_owned()
            .filter(|(_, v)| matches!(v.inner_type, EnumVariantType::Empty));
        let complex_variants = enumerated_variants
            .to_owned()
            .filter(|(_, v)| !matches!(v.inner_type, EnumVariantType::Empty));

        let simple_variant_checks = gen_simple_type_checks(simple_variants);
        let complex_variant_checks = gen_complex_type_checks(complex_variants);
        quote!(
            const is_$(obj_name.to_obj_identifier()) = (v) => {
                return $simple_variant_checks || $complex_variant_checks
            }
        )
    }

    fn gen_simple_type_checks<'a>(
        variants: impl Iterator<Item = (usize, &'a EnumVariant)> + Clone,
    ) -> Tokens {
        if variants.to_owned().count() == 0 {
            Tokens::new()
        } else {
            let variant_checks =
                and_chain(variants.map(|(_, variant)| quote!(v === $(quoted(&variant.name)))));
            let type_check = simple_enum_type_check();
            quote!(($type_check && $variant_checks))
        }
    }

    fn gen_complex_type_checks<'a>(
        variants: impl Iterator<Item = (usize, &'a EnumVariant)> + Clone,
    ) -> Tokens {
        if variants.to_owned().count() == 0 {
            Tokens::new()
        } else {
            let variant_checks = or_chain(variants.map(|(_, variant)| {
                let inner_type_checks = gen_variant_check(variant);
                quote!((v.key === $(quoted(&variant.name)) && $inner_type_checks))
            }));
            let type_check = complex_enum_type_check();
            quote!(($type_check && $variant_checks))
        }
    }

    fn gen_variant_check(variant: &EnumVariant) -> Tokens {
        match &variant.inner_type {
            EnumVariantType::Empty => unreachable!(),
            EnumVariantType::NewType(fields) => {
                let field_checks = gen_struct_field_available_checks(fields);
                let type_checks = gen_struct_field_type_checks(fields);
                quote!($field_checks && $type_checks)
            }
            EnumVariantType::Tuple(fields) => {
                let type_checks = gen_array_field_type_checks(fields);
                quote!(Array.isArray(v.inner) && v.inner.length === $(fields.len()) && $type_checks)
            }
        }
    }

    fn simple_enum_type_check() -> Tokens {
        quote!(typeof v === "string")
    }

    fn complex_enum_type_check() -> Tokens {
        quote!(typeof v === "object" && "key" in v && "inner" in v)
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
