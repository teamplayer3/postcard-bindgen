use genco::{lang::js::Tokens, quote, tokens::quoted};

use crate::{
    code_gen::{JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE},
    registry::{EnumVariant, EnumVariantType},
    utils::StrExt,
};

use super::{
    and_chain, gen_array_field_type_checks, gen_struct_field_available_checks,
    gen_struct_field_type_checks, or_chain, InnerTypeAccess,
};

pub fn gen_check_func(obj_name: impl AsRef<str>, variants: impl AsRef<[EnumVariant]>) -> Tokens {
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
        let variant_checks = and_chain(
            variants
                .map(|(_, variant)| quote!(v.$JS_ENUM_VARIANT_KEY === $(quoted(&variant.name)))),
        );
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
            quote!((v.$JS_ENUM_VARIANT_KEY === $(quoted(&variant.name)) && $inner_type_checks))
        }));
        let type_check = complex_enum_type_check();
        quote!(($type_check && $variant_checks))
    }
}

fn gen_variant_check(variant: &EnumVariant) -> Tokens {
    match &variant.inner_type {
        EnumVariantType::Empty => unreachable!(),
        EnumVariantType::NewType(fields) => {
            let field_checks =
                gen_struct_field_available_checks(fields, InnerTypeAccess::EnumInner);
            let type_checks = gen_struct_field_type_checks(fields, InnerTypeAccess::EnumInner);
            quote!($field_checks && $type_checks)
        }
        EnumVariantType::Tuple(fields) => {
            let type_checks = gen_array_field_type_checks(fields, InnerTypeAccess::EnumInner);
            quote!(Array.isArray(v.$JS_ENUM_VARIANT_VALUE) && v.$JS_ENUM_VARIANT_VALUE.length === $(fields.len()) && $type_checks)
        }
    }
}

fn simple_enum_type_check() -> Tokens {
    quote!(typeof v === "object" && $(quoted(JS_ENUM_VARIANT_KEY)))
}

fn complex_enum_type_check() -> Tokens {
    quote!(typeof v === "object" && $(quoted(JS_ENUM_VARIANT_KEY)) in v && $(quoted(JS_ENUM_VARIANT_VALUE)) in v)
}
