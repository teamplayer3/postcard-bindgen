use genco::{prelude::js::Tokens, quote, tokens::quoted};

use crate::{
    code_gen::js::{
        generateable::container::BindingTypeGenerateable,
        utils::{colon_chain, divider_chain, line_break_chain},
    },
    registry::BindingType,
};

pub fn gen_ts_typings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    quote!(
        $(gen_number_decls())

        $(gen_bindings_types(&bindings))

        $(gen_type_decl(&bindings))
        $(gen_value_type_decl(bindings))

        $(gen_ser_des_decls())
    )
}

fn gen_number_decls() -> Tokens {
    quote!(
        declare type u8 = number
        declare type u16 = number
        declare type u32 = number
        declare type u64 = number
        declare type u128 = number
        declare type usize = number
        declare type i8 = number
        declare type i16 = number
        declare type i32 = number
        declare type i64 = number
        declare type i128 = number
        declare type isize = number
    )
}

fn gen_type_decl(bindings: impl AsRef<[BindingType]>) -> Tokens {
    let type_cases = divider_chain(
        bindings
            .as_ref()
            .iter()
            .map(|b| quote!($(quoted(b.inner_name())))),
    );
    quote!(export type Type = $type_cases)
}

fn gen_value_type_decl(bindings: impl AsRef<[BindingType]>) -> Tokens {
    let if_cases = colon_chain(
        bindings
            .as_ref()
            .iter()
            .map(|b| quote!(T extends $(quoted(b.inner_name())) ? $(b.inner_name()))),
    );
    quote!(declare type ValueType<T extends Type> = $if_cases : void)
}

fn gen_ser_des_decls() -> Tokens {
    quote!(
        export function serialize<T extends Type>(type: T, value: ValueType<T>): u8[]
        export function deserialize<T extends Type>(type: T, bytes: u8[]): ValueType<T>
    )
}

fn gen_bindings_types(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_break_chain(bindings.as_ref().iter().map(gen_binding_type))
}

fn gen_binding_type(binding: &BindingType) -> Tokens {
    let name = binding.inner_name();
    let body = binding.gen_ts_typings_body();
    quote!(export type $name = $body)
}

#[cfg(test)]
mod test {
    use genco::quote;

    use crate::{
        code_gen::js::generateable::{container::BindingTypeGenerateable, types::JsTypeGenerateable},
        registry::{BindingType, EnumType, EnumVariant, EnumVariantType, StructField, StructType},
        type_info::{ArrayMeta, ValueType, NumberMeta, ObjectMeta, OptionalMeta, StringMeta},
        utils::assert_tokens,
    };

    use super::gen_binding_type;

    #[test]
    fn test_js_type_with_number_typings() {
        let number_combs = [
            (1, false),
            (2, false),
            (4, false),
            (8, false),
            (16, false),
            (1, true),
            (2, true),
            (4, true),
            (8, true),
            (16, true),
        ];
        let js_type = [
            quote!(u8),
            quote!(u16),
            quote!(u32),
            quote!(u64),
            quote!(u128),
            quote!(i8),
            quote!(i16),
            quote!(i32),
            quote!(i64),
            quote!(i128),
        ];
        let assert_combs = number_combs.iter().zip(js_type);

        for assertion in assert_combs.clone() {
            let ty = ValueType::Number(NumberMeta::Integer {
                bytes: assertion.0 .0,
                signed: assertion.0 .1,
            });
            assert_tokens(quote!($(ty.gen_ts_type())), assertion.1);
        }

        for assertion in assert_combs.clone() {
            let ty = ValueType::Array(ArrayMeta {
                items_type: Box::new(ValueType::Number(NumberMeta::Integer {
                    bytes: assertion.0 .0,
                    signed: assertion.0 .1,
                })),
            });

            assert_tokens(quote!($(ty.gen_ts_type())), quote!($(assertion.1)[]));
        }

        for assertion in assert_combs {
            let ty = ValueType::Optional(OptionalMeta {
                inner: Box::new(ValueType::Number(NumberMeta::Integer {
                    bytes: assertion.0 .0,
                    signed: assertion.0 .1,
                })),
            });

            assert_tokens(
                quote!($(ty.gen_ts_type())),
                quote!($(assertion.1) | undefined),
            );
        }
    }

    #[test]
    fn test_js_type_without_number_typings() {
        let ty = ValueType::Object(ObjectMeta { name: "A" });
        assert_tokens(quote!($(ty.gen_ts_type())), quote!(A));

        let ty = ValueType::String(StringMeta {});
        assert_tokens(quote!($(ty.gen_ts_type())), quote!(string));
    }

    #[test]
    fn test_struct_structure_typings() {
        let tokens = StructType {
            name: "A",
            fields: vec![
                StructField {
                    name: "a",
                    js_type: ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    }),
                },
                StructField {
                    name: "b",
                    js_type: ValueType::Object(ObjectMeta { name: "B" }),
                },
                StructField {
                    name: "c",
                    js_type: ValueType::String(StringMeta {}),
                },
                StructField {
                    name: "d",
                    js_type: ValueType::Array(ArrayMeta {
                        items_type: Box::new(ValueType::Number(NumberMeta::Integer {
                            bytes: 1,
                            signed: false,
                        })),
                    }),
                },
                StructField {
                    name: "e",
                    js_type: ValueType::Optional(OptionalMeta {
                        inner: Box::new(ValueType::Number(NumberMeta::Integer {
                            bytes: 1,
                            signed: false,
                        })),
                    }),
                },
            ],
        }
        .gen_ts_typings_body();

        assert_tokens(
            tokens,
            quote!({ a: u8, b: B, c: string, d: u8[], e: u8 | undefined }),
        )
    }

    #[test]
    fn test_struct_typings() {
        let test_binding = gen_binding_type(&BindingType::Struct(StructType {
            name: "A",
            fields: vec![StructField {
                name: "a",
                js_type: ValueType::Number(NumberMeta::Integer {
                    bytes: 1,
                    signed: false,
                }),
            }],
        }));

        assert_tokens(test_binding, quote!(export type A = { a: u8 }))
    }

    #[test]
    fn test_enum_typings() {
        let test_binding = gen_binding_type(&BindingType::Enum(EnumType {
            name: "A",
            variants: vec![
                EnumVariant {
                    name: "A",
                    index: 0,
                    inner_type: EnumVariantType::Empty,
                },
                EnumVariant {
                    name: "B",
                    index: 1,
                    inner_type: EnumVariantType::Tuple(vec![ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    })]),
                },
            ],
        }));

        assert_tokens(
            test_binding,
            quote!(export type A = { tag: "A" } | { tag: "B", value: u8 }),
        )
    }
}
