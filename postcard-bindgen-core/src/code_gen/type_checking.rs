use genco::{lang::js::Tokens, quote};

pub mod strukt {
    use genco::{lang::js::Tokens, quote, tokens::quoted};

    use crate::{
        registry::StructField,
        type_info::{JsType, ObjectMeta},
        StrExt, StringExt,
    };

    pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
        let obj_name = obj_name.as_ref();

        quote! {
            const is_$(obj_name.to_obj_identifier()) = (v) => {
                return typeof v === "object" && $(gen_field_checks(&fields).iter().chain(&gen_type_checks(fields)).map(|q| q.to_string().unwrap()).collect::<Vec<_>>().join("&&"))
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
                quote!(is_$(name.to_obj_identifier())(v.$(&field.name)))
            }
            _ => quote!(typeof v.$(field.name.as_str()) === $(quoted(field.js_type.to_string()))),
        }
    }
}

pub mod tuple_struct {
    use convert_case::{Case, Casing};
    use genco::{lang::js::Tokens, quote, tokens::quoted};

    use crate::type_info::{JsType, ObjectMeta};

    pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[JsType]>) -> Tokens {
        let obj_name = obj_name.as_ref();

        quote! {
            const is_$(obj_name.to_case(Case::Snake).to_uppercase()) = (v) => {
                return Array.isArray(v) && v.length === $(fields.as_ref().len()) && $(gen_type_checks(fields).iter().map(|q| q.to_string().unwrap()).collect::<Vec<_>>().join("&&"))
            }
        }
    }

    fn gen_type_checks(fields: impl AsRef<[JsType]>) -> Vec<Tokens> {
        fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(index, field)| gen_type_check(index, field))
            .collect::<Vec<_>>()
    }

    fn gen_type_check(index: usize, field: &JsType) -> Tokens {
        match field {
            JsType::Array(_) => quote!(Array.isArray(v[$index])),
            JsType::Object(ObjectMeta { name }) => {
                quote!(is_$(name.to_case(Case::Snake).to_uppercase())(v[$index]))
            }
            _ => quote!(typeof v[$index] === $(quoted(field.to_string()))),
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
        registry::{EnumVariant, EnumVariantType, StructField},
        type_info::{JsType, ObjectMeta},
        StrExt, StringExt,
    };

    use super::{and_chain, or_chain};

    pub fn gen_check_func(
        obj_name: impl AsRef<str>,
        variants: impl AsRef<[EnumVariant]>,
    ) -> Tokens {
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
            const is_$(obj_name.as_ref().to_obj_identifier()) = (v) => {
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
                let field_checks = and_chain(gen_field_checks(fields));
                let type_checks = and_chain(gen_type_checks(fields));
                quote!($field_checks && $type_checks)
            }
            EnumVariantType::Tuple(fields) => {
                let type_checks = and_chain(
                    fields
                        .iter()
                        .enumerate()
                        .map(|(index, field)| gen_type_check_array(index, field)),
                );
                quote!(Array.isArray(v.inner) && v.inner.length === $(fields.len()) && $type_checks)
            }
        }
    }

    fn gen_field_checks(fields: impl AsRef<[StructField]>) -> Vec<Tokens> {
        fields
            .as_ref()
            .iter()
            .map(|field| quote!( $(quoted(&field.name)) in v.inner))
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
                quote!(is_$(name.to_obj_identifier())(v.$(&field.name)))
            }
            _ => {
                quote!(typeof v.inner.$(field.name.as_str()) === $(quoted(field.js_type.to_string())))
            }
        }
    }

    fn gen_type_check_array(index: usize, field: &JsType) -> Tokens {
        match &field {
            JsType::Array(_) => quote!(Array.isArray(v[$index])),
            JsType::Object(ObjectMeta { name }) => {
                quote!(is_$(name.to_obj_identifier())(v[$index]))
            }
            _ => {
                quote!(typeof v.inner[$index] === $(quoted(field.to_string())))
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
