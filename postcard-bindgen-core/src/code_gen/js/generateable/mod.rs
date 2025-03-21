pub mod container;
pub mod types;

use core::borrow::Borrow;

use container::BindingTypeGenerateable;
use genco::{quote, quote_in, tokens::quoted};

use crate::{
    code_gen::{
        js::Tokens,
        utils::{ContainerFullQualifiedTypeBuilder, TokensIterExt},
    },
    registry::{Container, ContainerCollection, Module},
};

use super::GenerationSettings;

pub fn gen_ts_typings(
    containers: &ContainerCollection,
    gen_settings: impl Borrow<GenerationSettings>,
) -> Tokens {
    quote!(
        $(gen_number_decls())

        $(gen_extra_types_decls())

        $(gen_bindings_types(containers))

        $(gen_type_decl(containers.all_containers()))
        $(gen_value_type_decl(containers.all_containers()))

        $(gen_ser_des_decls(gen_settings.borrow().ser, gen_settings.borrow().des))
    )
}

fn gen_number_decls() -> Tokens {
    let types = [
        ("u8", "number"),
        ("u16", "number"),
        ("u32", "number"),
        ("u64", "bigint"),
        ("u128", "bigint"),
        ("usize", "bigint"),
        ("i8", "number"),
        ("i16", "number"),
        ("i32", "number"),
        ("i64", "bigint"),
        ("i128", "bigint"),
        ("isize", "bigint"),
        ("NonZeroU8", "number"),
        ("NonZeroU16", "number"),
        ("NonZeroU32", "number"),
        ("NonZeroU64", "bigint"),
        ("NonZeroU128", "bigint"),
        ("NonZeroUsize", "bigint"),
        ("NonZeroI8", "number"),
        ("NonZeroI16", "number"),
        ("NonZeroI32", "number"),
        ("NonZeroI64", "bigint"),
        ("NonZeroI128", "bigint"),
        ("NonZeroIsize", "bigint"),
        ("f32", "number"),
        ("f64", "number"),
    ];
    types
        .into_iter()
        .map(|(name, ty)| quote!(declare type $name = $ty))
        .join_with_line_breaks()
}

fn gen_extra_types_decls() -> Tokens {
    quote!(
        declare type ArrayLengthMutationKeys = "splice" | "push" | "pop" | "shift" | "unshift"
        declare type FixedLengthArray<T, L extends number, TObj = [T, ...Array<T>]> =
            Pick<TObj, Exclude<keyof TObj, ArrayLengthMutationKeys>>
            & {
                readonly length: L
                [ I : number ] : T
                [Symbol.iterator]: () => IterableIterator<T>
            }
    )
}

fn gen_type_decl(bindings: impl Iterator<Item = Container>) -> Tokens {
    let type_cases = bindings
        .map(|container| quote!($(quoted(ContainerFullQualifiedTypeBuilder::from(&container).build()))))
        .join_with_vertical_line();
    quote!(export type Type = $type_cases)
}

fn gen_value_type_decl(bindings: impl Iterator<Item = Container>) -> Tokens {
    let if_cases = bindings
        .map(|container| {
            let full_qualified = ContainerFullQualifiedTypeBuilder::from(&container).build();
            quote!(T extends $(quoted(&full_qualified)) ? $(full_qualified))
        })
        .join_with_colon();
    quote!(declare type ValueType<T extends Type> = $if_cases : void)
}

fn gen_ser_des_decls(ser: bool, des: bool) -> Tokens {
    quote!(
        $(if ser {
            export function serialize<T extends Type>(type: T, value: ValueType<T>): Uint8Array
        })

        $(if des {
            export interface Result<T extends Type> {
                value: ValueType<T>;
                bytes: Uint8Array;
            }

            export function deserialize<T extends Type>(type: T, bytes: Uint8Array): Result<T>
        })
    )
}

fn gen_bindings_types(containers: &ContainerCollection) -> Tokens {
    let (containers, mods) = containers.containers_per_module();

    let mut root_level = Tokens::new();

    for r#mod in mods {
        create_namespace(&mut root_level, r#mod);
    }

    let containers = containers
        .iter()
        .map(gen_binding_type)
        .join_with_line_breaks();

    root_level.append(containers);

    root_level
}

fn create_namespace(tokens: &mut Tokens, r#mod: Module<'_>) {
    let (containers, mods) = r#mod.entries();

    quote_in! {*tokens=>
        export namespace $(r#mod.name()) $("{")
    };

    tokens.push();
    tokens.indent();

    for r#mod in mods {
        create_namespace(tokens, r#mod);
    }

    let containers = containers
        .iter()
        .map(gen_binding_type)
        .join_with_line_breaks();

    tokens.append(containers);

    tokens.push();
    tokens.unindent();
    tokens.append("}");
    tokens.push();
}

fn gen_binding_type(binding: &Container) -> Tokens {
    let name = binding.name;
    let body = binding.r#type.gen_ts_typings_body();
    quote!(export type $name = $body)
}

#[cfg(test)]
mod test {
    use genco::quote;

    use crate::{
        code_gen::{
            js::generateable::{container::BindingTypeGenerateable, types::JsTypeGenerateable},
            utils::assert_tokens,
        },
        path::Path,
        registry::{
            BindingType, Container, EnumType, EnumVariant, EnumVariantType, StructField, StructType,
        },
        type_info::{ArrayMeta, NumberMeta, ObjectMeta, OptionalMeta, StringMeta, ValueType},
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
                zero_able: true,
            });
            assert_tokens(quote!($(ty.gen_ts_type())), assertion.1);
        }

        for assertion in assert_combs.clone() {
            let ty = ValueType::Array(ArrayMeta {
                items_type: Box::new(ValueType::Number(NumberMeta::Integer {
                    bytes: assertion.0 .0,
                    signed: assertion.0 .1,
                    zero_able: true,
                })),
                length: None,
                max_length: None,
            });

            assert_tokens(quote!($(ty.gen_ts_type())), quote!($(assertion.1)[]));
        }

        for assertion in assert_combs {
            let ty = ValueType::Optional(OptionalMeta {
                inner: Box::new(ValueType::Number(NumberMeta::Integer {
                    bytes: assertion.0 .0,
                    signed: assertion.0 .1,
                    zero_able: true,
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
        let ty = ValueType::Object(ObjectMeta {
            name: "A",
            path: Path::new("", "::"),
        });
        assert_tokens(quote!($(ty.gen_ts_type())), quote!(A));

        let ty = ValueType::String(StringMeta { max_length: None });
        assert_tokens(quote!($(ty.gen_ts_type())), quote!(string));
    }

    #[test]
    fn test_struct_structure_typings() {
        let tokens = StructType {
            fields: vec![
                StructField {
                    name: "a",
                    v_type: ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                        zero_able: true,
                    }),
                },
                StructField {
                    name: "b",
                    v_type: ValueType::Object(ObjectMeta {
                        name: "B",
                        path: Path::new("", "::"),
                    }),
                },
                StructField {
                    name: "c",
                    v_type: ValueType::String(StringMeta { max_length: None }),
                },
                StructField {
                    name: "d",
                    v_type: ValueType::Array(ArrayMeta {
                        items_type: Box::new(ValueType::Number(NumberMeta::Integer {
                            bytes: 1,
                            signed: false,
                            zero_able: true,
                        })),
                        length: None,
                        max_length: None,
                    }),
                },
                StructField {
                    name: "e",
                    v_type: ValueType::Optional(OptionalMeta {
                        inner: Box::new(ValueType::Number(NumberMeta::Integer {
                            bytes: 1,
                            signed: false,
                            zero_able: true,
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
        let test_binding = gen_binding_type(&Container {
            name: "A",
            path: Path::new("", "::"),
            r#type: BindingType::Struct(StructType {
                fields: vec![StructField {
                    name: "a",
                    v_type: ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                        zero_able: true,
                    }),
                }],
            }),
        });

        assert_tokens(test_binding, quote!(export type A = { a: u8 }))
    }

    #[test]
    fn test_enum_typings() {
        let test_binding = gen_binding_type(&Container {
            name: "A",
            path: Path::new("", "::"),
            r#type: BindingType::Enum(EnumType {
                variants: vec![
                    EnumVariant {
                        name: "A",
                        index: 0,
                        inner_type: EnumVariantType::Empty,
                    },
                    EnumVariant {
                        name: "B",
                        index: 1,
                        inner_type: EnumVariantType::Tuple(vec![ValueType::Number(
                            NumberMeta::Integer {
                                bytes: 1,
                                signed: false,
                                zero_able: true,
                            },
                        )]),
                    },
                ],
            }),
        });

        assert_tokens(
            test_binding,
            quote!(export type A = { tag: "A" } | { tag: "B", value: u8 }),
        )
    }
}
