use genco::quote;

use crate::{
    registry::{
        BindingType, EnumType, EnumVariant, EnumVariantType, StructField, StructType,
        TupleStructType, UnitStructType,
    },
    type_info::{ArrayMeta, ValueType, NumberMeta, ObjectMeta, OptionalMeta, StringMeta},
    utils::assert_tokens,
};

use super::gen_ser_des_functions;

#[test]
fn test_binding_struct_unit() {
    let binding = BindingType::UnitStruct(UnitStructType { name: "A" });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { }
            const deserialize_A = (d) => ({ })
        ),
    )
}

#[test]
fn test_binding_struct_new_type() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![ValueType::Number(NumberMeta::Integer {
            bytes: 1,
            signed: false,
        })],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES, false, v[0]) }
            const deserialize_A = (d) => ([d.deserialize_number(U8_BYTES, false)])
        ),
    )
}

#[test]
fn test_binding_struct_tuple_2() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![
            ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
            ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES, false, v[0]); s.serialize_number(U8_BYTES, false, v[1]) }
            const deserialize_A = (d) => ([d.deserialize_number(U8_BYTES, false), d.deserialize_number(U8_BYTES, false)])
        ),
    )
}

#[test]
fn test_binding_struct_tuple_3() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![
            ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
            ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
            ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES, false, v[0]); s.serialize_number(U8_BYTES, false, v[1]); s.serialize_number(U8_BYTES, false, v[2]) }
            const deserialize_A = (d) => ([d.deserialize_number(U8_BYTES, false), d.deserialize_number(U8_BYTES, false), d.deserialize_number(U8_BYTES, false)])
        ),
    )
}

#[test]
fn test_binding_struct_tuple_different_types() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![
            ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
            ValueType::String(StringMeta {}),
            ValueType::Optional(OptionalMeta {
                inner: Box::new(ValueType::Number(NumberMeta::Integer {
                    bytes: 1,
                    signed: false,
                })),
            }),
            ValueType::Array(ArrayMeta {
                items_type: Box::new(ValueType::Number(NumberMeta::Integer {
                    bytes: 1,
                    signed: false,
                })),
            }),
            ValueType::Object(ObjectMeta { name: "A" }),
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES, false, v[0]); s.serialize_string(v[1]); if (v[2] !== undefined) { s.serialize_number(U32_BYTES, false, 1); s.serialize_number(U8_BYTES, false, v[2]) } else { s.serialize_number(U32_BYTES, false, 0) }; s.serialize_array((s, v) => s.serialize_number(U8_BYTES, false, v), v[3]); serialize_A(s, v[4]) }
            const deserialize_A = (d) => ([d.deserialize_number(U8_BYTES, false), d.deserialize_string(), (d.deserialize_number(U32_BYTES, false) === 0) ? undefined : d.deserialize_number(U8_BYTES, false), d.deserialize_array(() => d.deserialize_number(U8_BYTES, false)), deserialize_A(d)])
        ),
    )
}

#[test]
fn test_binding_struct() {
    let binding = BindingType::Struct(StructType {
        name: "A",
        fields: vec![StructField {
            name: "a",
            v_type: ValueType::Number(NumberMeta::Integer {
                bytes: 1,
                signed: false,
            }),
        }],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES, false, v.a) }
            const deserialize_A = (d) => ({ a: d.deserialize_number(U8_BYTES, false) })
        ),
    );
}

#[test]
fn test_binding_struct_different_types() {
    let binding = BindingType::Struct(StructType {
        name: "A",
        fields: vec![
            StructField {
                name: "a",
                v_type: ValueType::Number(NumberMeta::Integer {
                    bytes: 1,
                    signed: false,
                }),
            },
            StructField {
                name: "b",
                v_type: ValueType::String(StringMeta {}),
            },
            StructField {
                name: "c",
                v_type: ValueType::Optional(OptionalMeta {
                    inner: Box::new(ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    })),
                }),
            },
            StructField {
                name: "d",
                v_type: ValueType::Array(ArrayMeta {
                    items_type: Box::new(ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    })),
                }),
            },
            StructField {
                name: "e",
                v_type: ValueType::Object(ObjectMeta { name: "A" }),
            },
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES, false, v.a); s.serialize_string(v.b); if (v.c !== undefined) { s.serialize_number(U32_BYTES, false, 1); s.serialize_number(U8_BYTES, false, v.c) } else { s.serialize_number(U32_BYTES, false, 0) }; s.serialize_array((s, v) => s.serialize_number(U8_BYTES, false, v), v.d); serialize_A(s, v.e) }
            const deserialize_A = (d) => ({ a: d.deserialize_number(U8_BYTES, false), b: d.deserialize_string(), c: (d.deserialize_number(U32_BYTES, false) === 0) ? undefined : d.deserialize_number(U8_BYTES, false), d: d.deserialize_array(() => d.deserialize_number(U8_BYTES, false)), e: deserialize_A(d) })
        ),
    );
}

#[test]
fn test_binding_enum() {
    let binding = BindingType::Enum(EnumType {
        name: "A",
        variants: vec![
            EnumVariant {
                index: 0,
                name: "A",
                inner_type: EnumVariantType::Empty,
            },
            EnumVariant {
                index: 1,
                name: "B",
                inner_type: EnumVariantType::NewType(vec![StructField {
                    name: "a",
                    v_type: ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    }),
                }]),
            },
            EnumVariant {
                index: 2,
                name: "C",
                inner_type: EnumVariantType::Tuple(vec![ValueType::Number(NumberMeta::Integer {
                    bytes: 1,
                    signed: false,
                })]),
            },
            EnumVariant {
                index: 3,
                name: "D",
                inner_type: EnumVariantType::Tuple(vec![
                    ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    }),
                    ValueType::Number(NumberMeta::Integer {
                        bytes: 1,
                        signed: false,
                    }),
                ]),
            },
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { switch (v.tag) { case "A": s.serialize_number(U32_BYTES, false, 0); break; case "B": s.serialize_number(U32_BYTES, false, 1); s.serialize_number(U8_BYTES, false, v.value.a); break; case "C": s.serialize_number(U32_BYTES, false, 2); s.serialize_number(U8_BYTES, false, v.value); break; case "D": s.serialize_number(U32_BYTES, false, 3); s.serialize_number(U8_BYTES, false, v.value[0]); s.serialize_number(U8_BYTES, false, v.value[1]); break } }
            const deserialize_A = (d) => { switch (d.deserialize_number(U32_BYTES, false)) { case 0: return { tag: "A" }; case 1: return { tag: "B" , value: { a: d.deserialize_number(U8_BYTES, false) } }; case 2: return { tag: "C" , value: d.deserialize_number(U8_BYTES, false) }; case 3: return { tag: "D" , value: [d.deserialize_number(U8_BYTES, false), d.deserialize_number(U8_BYTES, false)] }; default: throw "variant not implemented" } }
        ),
    )
}
