use genco::quote;

use crate::{
    registry::{BindingType, StructField, StructType, TupleStructType, UnitStructType},
    type_info::{ArrayMeta, JsType, NumberMeta, ObjectMeta, StringMeta},
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
        fields: vec![JsType::Number(NumberMeta {
            bytes: 1,
            signed: false,
        })],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES,false,v[0]) }
            const deserialize_A = (d) => ([ d.deserialize_number(U8_BYTES,false) ])
        ),
    )
}

#[test]
fn test_binding_struct_tuple_2() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![
            JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
            JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES,false,v[0]); s.serialize_number(U8_BYTES,false,v[1]) }
            const deserialize_A = (d) => ([ d.deserialize_number(U8_BYTES,false), d.deserialize_number(U8_BYTES,false) ])
        ),
    )
}

#[test]
fn test_binding_struct_tuple_3() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![
            JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
            JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
            JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES,false,v[0]); s.serialize_number(U8_BYTES,false,v[1]); s.serialize_number(U8_BYTES,false,v[2]) }
            const deserialize_A = (d) => ([ d.deserialize_number(U8_BYTES,false), d.deserialize_number(U8_BYTES,false), d.deserialize_number(U8_BYTES,false) ])
        ),
    )
}

#[test]
fn test_binding_struct_tuple_different_types() {
    let binding = BindingType::TupleStruct(TupleStructType {
        name: "A",
        fields: vec![
            JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
            JsType::String(StringMeta {}),
            JsType::Optional(Box::new(JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }))),
            JsType::Array(ArrayMeta {
                items_type: Box::new(JsType::Number(NumberMeta {
                    bytes: 1,
                    signed: false,
                })),
            }),
            JsType::Object(ObjectMeta { name: "A" }),
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES,false,v[0]); s.serialize_string(v[1]); if (v[2] !== undefined) { s.serialize_number(U32_BYTES, false, 1); s.serialize_number(U8_BYTES,false,v[2]) } else { s.serialize_number(U32_BYTES, false, 0) }; s.serialize_array((s, v) => s.serialize_number(U8_BYTES,false,v),v[3]); serialize_A(s, v[4]) }
            const deserialize_A = (d) => ([ d.deserialize_number(U8_BYTES,false), d.deserialize_string(), (d.deserialize_number(U32_BYTES, false) === 0) ? undefined : d.deserialize_number(U8_BYTES,false), d.deserialize_array(() => d.deserialize_number(U8_BYTES,false)), deserialize_A(d) ])
        ),
    )
}

#[test]
fn test_binding_struct() {
    let binding = BindingType::Struct(StructType {
        name: "A",
        fields: vec![StructField {
            name: "a",
            js_type: JsType::Number(NumberMeta {
                bytes: 1,
                signed: false,
            }),
        }],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES,false,v.a) }
            const deserialize_A = (d) => ({ a: d.deserialize_number(U8_BYTES,false) })
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
                js_type: JsType::Number(NumberMeta {
                    bytes: 1,
                    signed: false,
                }),
            },
            StructField {
                name: "b",
                js_type: JsType::String(StringMeta {}),
            },
            StructField {
                name: "c",
                js_type: JsType::Optional(Box::new(JsType::Number(NumberMeta {
                    bytes: 1,
                    signed: false,
                }))),
            },
            StructField {
                name: "d",
                js_type: JsType::Array(ArrayMeta {
                    items_type: Box::new(JsType::Number(NumberMeta {
                        bytes: 1,
                        signed: false,
                    })),
                }),
            },
            StructField {
                name: "e",
                js_type: JsType::Object(ObjectMeta { name: "A" }),
            },
        ],
    });

    assert_tokens(
        gen_ser_des_functions(vec![binding]),
        quote!(
            const serialize_A = (s, v) => { s.serialize_number(U8_BYTES,false,v.a); s.serialize_string(v.b); if (v.c !== undefined) { s.serialize_number(U32_BYTES, false, 1); s.serialize_number(U8_BYTES,false,v.c) } else { s.serialize_number(U32_BYTES, false, 0) }; s.serialize_array((s, v) => s.serialize_number(U8_BYTES,false,v),v.d); serialize_A(s, v.e) }
            const deserialize_A = (d) => ({ a: d.deserialize_number(U8_BYTES,false), b: d.deserialize_string(), c: (d.deserialize_number(U32_BYTES, false) === 0) ? undefined : d.deserialize_number(U8_BYTES,false), d: d.deserialize_array(() => d.deserialize_number(U8_BYTES,false)), e: deserialize_A(d) })
        ),
    );
}
