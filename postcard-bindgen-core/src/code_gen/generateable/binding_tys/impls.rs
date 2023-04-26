use genco::quote;

use crate::{
    code_gen::{
        generateable::VariablePath,
        utils::{wrapped_brackets, wrapped_curly_brackets},
        JS_OBJECT_VARIABLE,
    },
    registry::{BindingType, EnumType, StructType, TupleStructType, UnitStructType},
};

use super::{des, ser, ts, ty_check, BindingTypeGenerateable};

impl BindingTypeGenerateable for BindingType {
    fn gen_ser_body(&self) -> genco::prelude::js::Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ser_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ser_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ser_body(),
            Self::Enum(enum_type) => enum_type.gen_ser_body(),
        }
    }

    fn gen_des_body(&self) -> genco::prelude::js::Tokens {
        match self {
            Self::Struct(struct_type) => wrapped_brackets(struct_type.gen_des_body()),
            Self::UnitStruct(unit_struct_type) => wrapped_brackets(unit_struct_type.gen_des_body()),
            Self::TupleStruct(tuple_struct_type) => {
                wrapped_brackets(tuple_struct_type.gen_des_body())
            }
            Self::Enum(enum_type) => wrapped_curly_brackets(enum_type.gen_des_body()),
        }
    }

    fn gen_ty_check_body(&self) -> genco::prelude::js::Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ty_check_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ty_check_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ty_check_body(),
            Self::Enum(enum_type) => enum_type.gen_ty_check_body(),
        }
    }

    fn gen_ts_typings_body(&self) -> genco::prelude::js::Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ts_typings_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ts_typings_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ts_typings_body(),
            Self::Enum(enum_type) => enum_type.gen_ts_typings_body(),
        }
    }
}

impl BindingTypeGenerateable for StructType {
    fn gen_ser_body(&self) -> genco::prelude::js::Tokens {
        ser::gen_accessors_fields(&self.fields, VariablePath::default())
    }

    fn gen_des_body(&self) -> genco::prelude::js::Tokens {
        des::gen_accessors_fields(&self.fields)
    }

    fn gen_ty_check_body(&self) -> genco::prelude::js::Tokens {
        ty_check::gen_object_checks(&self.fields, VariablePath::default())
    }

    fn gen_ts_typings_body(&self) -> genco::prelude::js::Tokens {
        ts::gen_typings_fields(&self.fields)
    }
}

impl BindingTypeGenerateable for TupleStructType {
    fn gen_ser_body(&self) -> genco::prelude::js::Tokens {
        ser::gen_accessors_indexed(&self.fields, VariablePath::default())
    }

    fn gen_des_body(&self) -> genco::prelude::js::Tokens {
        des::gen_accessors_indexed(&self.fields)
    }

    fn gen_ty_check_body(&self) -> genco::prelude::js::Tokens {
        ty_check::gen_array_checks(&self.fields, VariablePath::default())
    }

    fn gen_ts_typings_body(&self) -> genco::prelude::js::Tokens {
        ts::gen_typings_indexed(&self.fields)
    }
}

impl BindingTypeGenerateable for UnitStructType {
    fn gen_ser_body(&self) -> genco::prelude::js::Tokens {
        ser::gen_accessors_fields([], VariablePath::default())
    }

    fn gen_des_body(&self) -> genco::prelude::js::Tokens {
        des::gen_accessors_fields([])
    }

    fn gen_ty_check_body(&self) -> genco::prelude::js::Tokens {
        quote!(typeof $JS_OBJECT_VARIABLE === "object" && Object.keys($JS_OBJECT_VARIABLE).length === 0)
    }

    fn gen_ts_typings_body(&self) -> genco::prelude::js::Tokens {
        ts::gen_typings_fields([])
    }
}

impl BindingTypeGenerateable for EnumType {
    fn gen_ser_body(&self) -> genco::prelude::js::Tokens {
        enum_ty::ser::gen_function(&self.variants)
    }

    fn gen_des_body(&self) -> genco::prelude::js::Tokens {
        enum_ty::des::gen_function(&self.variants)
    }

    fn gen_ty_check_body(&self) -> genco::prelude::js::Tokens {
        enum_ty::ty_check::gen_check_func(&self.variants)
    }

    fn gen_ts_typings_body(&self) -> genco::prelude::js::Tokens {
        enum_ty::ts::gen_typings(&self.variants)
    }
}

pub mod enum_ty {
    pub mod ser {

        use genco::{
            lang::js::Tokens,
            prelude::JavaScript,
            quote, quote_in,
            tokens::{quoted, FormatInto},
        };

        use crate::{
            code_gen::{
                generateable::{
                    binding_tys::ser, types::JsTypeGenerateable, VariableAccess, VariablePath,
                },
                utils::semicolon_chain,
                JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE,
            },
            registry::{EnumVariant, EnumVariantType},
        };

        pub fn gen_function(variants: impl AsRef<[EnumVariant]>) -> Tokens {
            let enumerated_variants = variants.as_ref().iter().enumerate();
            let switch_body = semicolon_chain(
                enumerated_variants.map(|(index, variant)| gen_case_for_variant(index, variant)),
            );
            quote!(switch (v.$JS_ENUM_VARIANT_KEY) { $switch_body })
        }

        enum CaseBody {
            Body(Tokens),
            None,
        }

        impl FormatInto<JavaScript> for CaseBody {
            fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
                quote_in! { *tokens =>
                    $(match self {
                        CaseBody::Body(b) => $b;,
                        CaseBody::None => ()
                    })
                }
            }
        }

        fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Tokens {
            let variant_name = quoted(variant.name);
            let variable_path = VariablePath::default()
                .modify_push(VariableAccess::Field(JS_ENUM_VARIANT_VALUE.into()));
            let body = match &variant.inner_type {
                EnumVariantType::Empty => CaseBody::None,
                EnumVariantType::Tuple(fields) => CaseBody::Body(match fields.len() {
                    1 => fields[0].gen_ser_accessor(variable_path),
                    _ => ser::gen_accessors_indexed(fields, variable_path),
                }),
                EnumVariantType::NewType(fields) => {
                    CaseBody::Body(ser::gen_accessors_fields(fields, variable_path))
                }
            };

            quote!(case $variant_name: s.serialize_number(U32_BYTES, false, $index); $body break)
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
                generateable::{
                    binding_tys::des,
                    types::{self, JsTypeGenerateable},
                },
                utils::semicolon_chain,
                JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE,
            },
            registry::{EnumVariant, EnumVariantType},
        };

        pub fn gen_function(variants: impl AsRef<[EnumVariant]>) -> Tokens {
            let enumerated_variants = variants.as_ref().iter().enumerate();
            let switch_body = semicolon_chain(
                enumerated_variants.map(|(index, variant)| gen_case_for_variant(index, variant)),
            );
            quote!(switch (d.deserialize_number(U32_BYTES, false)) { $switch_body; default: throw "variant not implemented" })
        }

        enum CaseBody {
            Body(Tokens),
            None,
        }

        impl FormatInto<JavaScript> for CaseBody {
            fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
                quote_in! { *tokens =>
                    $(match self {
                        CaseBody::Body(b) => {, $JS_ENUM_VARIANT_VALUE: $b},
                        CaseBody::None => ()
                    })
                }
            }
        }

        fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Tokens {
            let variant_name = quoted(variant.name);
            let body = match &variant.inner_type {
                EnumVariantType::Empty => CaseBody::None,
                EnumVariantType::NewType(fields) => {
                    CaseBody::Body(des::gen_accessors_fields(fields))
                }
                EnumVariantType::Tuple(fields) => CaseBody::Body(match fields.len() {
                    1 => fields[0].gen_des_accessor(types::des::FieldAccessor::None),
                    _ => des::gen_accessors_indexed(fields),
                }),
            };
            quote!(case $index: return { $JS_ENUM_VARIANT_KEY: $variant_name $body })
        }
    }

    pub mod ty_check {
        use genco::{lang::js::Tokens, quote, tokens::quoted};

        use crate::{
            code_gen::{
                generateable::{
                    binding_tys::ty_check, types::JsTypeGenerateable, VariableAccess, VariablePath,
                },
                utils::{and_chain, or_chain},
                JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE, JS_OBJECT_VARIABLE,
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

            or_chain(
                [simple_variant_checks, complex_variant_checks]
                    .into_iter()
                    .flatten(),
            )
        }

        fn gen_simple_type_checks<'a>(
            variants: impl Iterator<Item = (usize, &'a EnumVariant)> + Clone,
        ) -> Option<Tokens> {
            if variants.to_owned().count() == 0 {
                None
            } else {
                let variant_checks = and_chain(variants.map(
                    |(_, variant)| quote!(v.$JS_ENUM_VARIANT_KEY === $(quoted(variant.name))),
                ));
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
                let variant_checks = or_chain(variants.map(|(_, variant)| {
            let inner_type_checks = gen_variant_check(variant);
            quote!((v.$JS_ENUM_VARIANT_KEY === $(quoted(variant.name)) && $inner_type_checks))
        }));
                let type_check = complex_enum_type_check();
                Some(quote!(($type_check && $variant_checks)))
            }
        }

        fn gen_variant_check(variant: &EnumVariant) -> Tokens {
            let variable_path = VariablePath::new("v".into())
                .modify_push(VariableAccess::Field(JS_ENUM_VARIANT_VALUE.into()));
            match &variant.inner_type {
                EnumVariantType::Empty => unreachable!(),
                EnumVariantType::NewType(fields) => {
                    ty_check::gen_object_checks(fields, variable_path)
                }
                EnumVariantType::Tuple(fields) => match fields.len() {
                    1 => fields[0].gen_ty_check(variable_path),
                    _ => ty_check::gen_array_checks(fields, variable_path),
                },
            }
        }

        fn simple_enum_type_check() -> Tokens {
            quote!(typeof $JS_OBJECT_VARIABLE === "object" && $(quoted(JS_ENUM_VARIANT_KEY)))
        }

        fn complex_enum_type_check() -> Tokens {
            quote!(typeof $JS_OBJECT_VARIABLE === "object" && $(quoted(JS_ENUM_VARIANT_KEY)) in $JS_OBJECT_VARIABLE && $(quoted(JS_ENUM_VARIANT_VALUE)) in $JS_OBJECT_VARIABLE)
        }
    }

    pub mod ts {
        use genco::{prelude::js::Tokens, quote, tokens::quoted};

        use crate::{
            code_gen::{
                generateable::{binding_tys, types::JsTypeGenerateable},
                utils::divider_chain,
                JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE,
            },
            registry::{EnumVariant, EnumVariantType},
        };

        pub fn gen_typings(variants: impl AsRef<[EnumVariant]>) -> Tokens {
            let body = divider_chain(variants.as_ref().iter().map(gen_variant_typings));
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
                            _ => binding_tys::ts::gen_typings_indexed(t),
                        },
                        EnumVariantType::NewType(n) => binding_tys::ts::gen_typings_fields(n),
                        _ => unreachable!(),
                    };
                    quote!({ $JS_ENUM_VARIANT_KEY: $name, $JS_ENUM_VARIANT_VALUE: $body })
                }
            }
        }
    }
}
