use genco::quote;

use crate::{code_gen::js::Tokens, registry::EnumType};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for EnumType {
    fn gen_ser_body(&self) -> Tokens {
        quote!($(ser::gen_function(&self.variants)))
    }

    fn gen_des_body(&self) -> Tokens {
        quote!($(des::gen_function(&self.variants)))
    }

    fn gen_ty_check_body(&self) -> Tokens {
        let body = ty_check::gen_check_func(&self.variants);
        quote!(return $body)
    }

    fn gen_ts_typings_body(&self) -> Tokens {
        ts::gen_typings(&self.variants)
    }
}

pub mod ser {

    use genco::{
        lang::js::Tokens,
        prelude::JavaScript,
        quote, quote_in,
        tokens::{quoted, FormatInto},
    };

    use crate::{
        code_gen::{
            js::{
                generateable::{container::ser, types::JsTypeGenerateable},
                Case, SwitchCase, VariableAccess, VariablePath, JS_ENUM_VARIANT_KEY,
                JS_ENUM_VARIANT_VALUE,
            },
            switch_case::DefaultCase,
        },
        registry::{EnumVariant, EnumVariantType},
    };

    pub fn gen_function(variants: impl AsRef<[EnumVariant]>) -> impl FormatInto<JavaScript> {
        let enumerated_variants = variants.as_ref().iter().enumerate();

        let mut switch_case = SwitchCase::new(quote!(v.$JS_ENUM_VARIANT_KEY));
        switch_case.extend_cases(
            enumerated_variants.map(|(index, variant)| gen_case_for_variant(index, variant)),
        );
        switch_case.default_case(DefaultCase::new_without_break(
            quote!(throw "variant not implemented"),
        ));

        switch_case
    }

    enum CaseBody {
        Body(Tokens),
        None,
    }

    impl FormatInto<JavaScript> for CaseBody {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    CaseBody::Body(b) => $b,
                    CaseBody::None => ()
                })
            }
        }
    }

    fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Case {
        let variant_name = quoted(variant.name);
        let variable_path = VariablePath::default()
            .modify_push(VariableAccess::Field(JS_ENUM_VARIANT_VALUE.into()));
        let body = match &variant.inner_type {
            EnumVariantType::Empty => CaseBody::None,
            EnumVariantType::Tuple(fields) => CaseBody::Body(match fields.len() {
                1 => quote!($(fields[0].gen_ser_accessor(variable_path));),
                _ => ser::gen_accessors_indexed(fields, variable_path),
            }),
            EnumVariantType::NewType(fields) => {
                CaseBody::Body(ser::gen_accessors_fields(fields, variable_path))
            }
        };

        Case::new(
            variant_name,
            quote! {
                s.serialize_number(U32_BYTES, false, $index);
                $body
            },
        )
    }
}

pub mod des {
    use genco::{
        lang::js::Tokens,
        prelude::JavaScript,
        quote, quote_in,
        tokens::{quoted, FormatInto},
    };

    use crate::{
        code_gen::{
            js::{
                generateable::{container::des, types::JsTypeGenerateable},
                Case, DefaultCase, FieldAccessor, SwitchCase, JS_ENUM_VARIANT_KEY,
                JS_ENUM_VARIANT_VALUE,
            },
            utils::{JoinType, TokensIterExt},
        },
        registry::{EnumVariant, EnumVariantType},
    };

    pub fn gen_function(variants: impl AsRef<[EnumVariant]>) -> impl FormatInto<JavaScript> {
        let enumerated_variants = variants.as_ref().iter().enumerate();

        let mut switch_case = SwitchCase::new(quote!(d.deserialize_number(U32_BYTES, false)));
        switch_case.extend_cases(
            enumerated_variants.map(|(index, variant)| gen_case_for_variant(index, variant)),
        );
        switch_case.default_case(DefaultCase::new_without_break(
            quote!(throw "variant not implemented"),
        ));

        switch_case
    }

    enum CaseBody {
        Body(Tokens),
        None,
    }

    impl FormatInto<JavaScript> for CaseBody {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    CaseBody::Body(b) => {$JS_ENUM_VARIANT_VALUE: $b},
                    CaseBody::None => ()
                })
            }
        }
    }

    fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Case {
        let variant_name = quoted(variant.name);
        let body = match &variant.inner_type {
            EnumVariantType::Empty => CaseBody::None,
            EnumVariantType::NewType(fields) => CaseBody::Body(des::gen_accessors_fields(fields)),
            EnumVariantType::Tuple(fields) => CaseBody::Body(match fields.len() {
                1 => fields[0].gen_des_accessor(FieldAccessor::None),
                _ => des::gen_accessors_indexed(fields),
            }),
        };

        let body = [quote!($JS_ENUM_VARIANT_KEY: $variant_name), quote!($body)]
            .into_iter()
            .join_with([JoinType::Comma, JoinType::LineBreak]);

        Case::new_without_break(
            index,
            quote! {
                return {
                    $body
                };
            },
        )
    }
}

pub mod ty_check {
    use genco::{lang::js::Tokens, quote, tokens::quoted};

    use crate::{
        code_gen::{
            js::{
                generateable::{container::ty_check, types::JsTypeGenerateable},
                VariableAccess, VariablePath, JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE,
                JS_OBJECT_VARIABLE,
            },
            utils::TokensIterExt,
        },
        registry::{EnumVariant, EnumVariantType},
    };

    pub fn gen_check_func(variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let enumerated_variants = variants.as_ref().iter().enumerate();
        let simple_variants = enumerated_variants
            .to_owned()
            .filter(|(_, v)| matches!(v.inner_type, EnumVariantType::Empty));
        let complex_variants = enumerated_variants
            .to_owned()
            .filter(|(_, v)| !matches!(v.inner_type, EnumVariantType::Empty));

        let simple_variant_checks = gen_simple_type_checks(simple_variants);
        let complex_variant_checks = gen_complex_type_checks(complex_variants);

        [simple_variant_checks, complex_variant_checks]
            .into_iter()
            .flatten()
            .join_logic_or()
    }

    fn gen_simple_type_checks<'a>(
        variants: impl Iterator<Item = (usize, &'a EnumVariant)> + Clone,
    ) -> Option<Tokens> {
        if variants.to_owned().count() == 0 {
            None
        } else {
            let variant_checks = variants
                .map(|(_, variant)| quote!(v.$JS_ENUM_VARIANT_KEY === $(quoted(variant.name))))
                .join_logic_or();
            let type_check = simple_enum_type_check();
            Some(quote!(($type_check && $variant_checks)))
        }
    }

    fn gen_complex_type_checks<'a>(
        variants: impl Iterator<Item = (usize, &'a EnumVariant)> + Clone,
    ) -> Option<Tokens> {
        if variants.to_owned().count() == 0 {
            None
        } else {
            let variant_checks = variants.map(|(_, variant)| {
                let inner_type_checks = gen_variant_check(variant);
                quote!((v.$JS_ENUM_VARIANT_KEY === $(quoted(variant.name)) && $inner_type_checks))
            }).join_logic_or();
            let type_check = complex_enum_type_check();
            Some(quote!(($type_check && $variant_checks)))
        }
    }

    fn gen_variant_check(variant: &EnumVariant) -> Tokens {
        let variable_path = VariablePath::new("v".into())
            .modify_push(VariableAccess::Field(JS_ENUM_VARIANT_VALUE.into()));
        match &variant.inner_type {
            EnumVariantType::Empty => unreachable!(),
            EnumVariantType::NewType(fields) => ty_check::gen_object_checks(fields, variable_path),
            EnumVariantType::Tuple(fields) => match fields.len() {
                1 => fields[0].gen_ty_check(variable_path),
                _ => ty_check::gen_array_checks(fields, variable_path),
            },
        }
    }

    fn simple_enum_type_check() -> Tokens {
        quote!(typeof $JS_OBJECT_VARIABLE === "object" && $(quoted(JS_ENUM_VARIANT_KEY)) in $JS_OBJECT_VARIABLE)
    }

    fn complex_enum_type_check() -> Tokens {
        quote!(typeof $JS_OBJECT_VARIABLE === "object" && $(quoted(JS_ENUM_VARIANT_KEY)) in $JS_OBJECT_VARIABLE && $(quoted(JS_ENUM_VARIANT_VALUE)) in $JS_OBJECT_VARIABLE)
    }
}

pub mod ts {
    use genco::{prelude::js::Tokens, quote, tokens::quoted};

    use crate::{
        code_gen::{
            js::{
                generateable::{container, types::JsTypeGenerateable},
                JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE,
            },
            utils::TokensIterExt,
        },
        registry::{EnumVariant, EnumVariantType},
    };

    pub fn gen_typings(variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let body = variants
            .as_ref()
            .iter()
            .map(gen_variant_typings)
            .join_with_vertical_line();
        quote!($body)
    }

    fn gen_variant_typings(variant: &EnumVariant) -> Tokens {
        let name = quoted(variant.name);
        match &variant.inner_type {
            EnumVariantType::Empty => quote!({ $JS_ENUM_VARIANT_KEY: $name }),
            t => {
                let body = match t {
                    EnumVariantType::Tuple(t) => match t.len() {
                        1 => t[0].gen_ts_type(),
                        _ => container::ts::gen_typings_indexed(t),
                    },
                    EnumVariantType::NewType(n) => container::ts::gen_typings_fields(n),
                    _ => unreachable!(),
                };
                quote!({ $JS_ENUM_VARIANT_KEY: $name, $JS_ENUM_VARIANT_VALUE: $body })
            }
        }
    }
}
