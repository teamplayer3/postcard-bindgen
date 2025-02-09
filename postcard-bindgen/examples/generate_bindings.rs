use std::{collections::HashMap, io::Write, num::NonZero, ops::Range, str::FromStr, u8};

use postcard_bindgen::{generate_bindings, javascript, python, PackageInfo, PostcardBindings};
use serde::Serialize;

#[derive(Debug, Serialize, PostcardBindings)]
struct UnionContainer;

#[derive(Debug, Serialize, PostcardBindings)]
struct StructContainer {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(Debug, Serialize, PostcardBindings)]
enum EnumContainer {
    A,
    B(u8),
    C(u8, StructContainer),
    D { a: u8, b: StructContainer },
}

#[derive(Debug, Serialize, PostcardBindings)]
struct TupleContainer(u8, StructContainer, EnumContainer);

#[derive(Debug, Serialize, PostcardBindings)]
struct ContainerTypes {
    u: UnionContainer,
    e_a: EnumContainer,
    e_b: EnumContainer,
    e_c: EnumContainer,
    e_d: EnumContainer,
    t: TupleContainer,
    s: StructContainer,
}

#[derive(Debug, Serialize, PostcardBindings)]
struct PrimitiveTypes {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    usize: usize,
    i8_max: i8,
    i8_min: i8,
    i16_max: i16,
    i16_min: i16,
    i32_max: i32,
    i32_min: i32,
    i64_max: i64,
    i64_min: i64,
    i128_max: i128,
    i128_min: i128,
    isize_max: isize,
    isize_min: isize,
    f32: f32,
    f64: f64,
    bool_true: bool,
    bool_false: bool,
    // TODO: char is not supported yet
    // char: char,
    none_zero: NonZero<u8>,
}

#[derive(Debug, Serialize, PostcardBindings)]
struct CompoundTypes {
    static_byte_slice: &'static [u8],
    static_str: &'static str,
    array: [u8; 10],
    range: Range<u16>,
    option_some: Option<u8>,
    option_none: Option<u8>,
    tuple: (u8, StructContainer, EnumContainer, TupleContainer),
}

#[derive(Debug, Serialize, PostcardBindings)]
struct AllocTypes {
    a: Vec<u8>,
    b: String,
    c: HashMap<u16, u8>,
}

#[derive(Debug, Serialize, PostcardBindings)]
struct HeaplessTypes {
    a: heapless::Vec<u8, 10>,
    b: heapless::String<10>,
    c: heapless::LinearMap<u8, u16, 10>,
}

#[derive(Debug, Serialize, PostcardBindings)]
struct AllTests {
    a: ContainerTypes,
    b: PrimitiveTypes,
    c: CompoundTypes,
    d: AllocTypes,
    e: HeaplessTypes,
    f: e::E,
}

mod e {
    use super::*;

    #[derive(Debug, Serialize, PostcardBindings)]
    pub struct E(pub u8, pub f::F);

    pub mod f {
        use super::*;

        #[derive(Debug, Serialize, PostcardBindings)]
        pub struct F(pub u8);
    }
}

fn main() {
    javascript::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "js-test-bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        javascript::GenerationSettings::enable_all()
            .runtime_type_checks(true)
            .esm_module(false)
            .module_structure(true),
        generate_bindings!(
            UnionContainer,
            StructContainer,
            EnumContainer,
            TupleContainer,
            ContainerTypes,
            PrimitiveTypes,
            CompoundTypes,
            AllocTypes,
            HeaplessTypes,
            AllTests,
            e::E,
            e::f::F
        ),
    )
    .unwrap();

    python::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "py-test-bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        python::GenerationSettings::enable_all()
            .runtime_type_checks(true)
            .module_structure(true),
        generate_bindings!(
            UnionContainer,
            StructContainer,
            EnumContainer,
            TupleContainer,
            ContainerTypes,
            PrimitiveTypes,
            CompoundTypes,
            AllocTypes,
            HeaplessTypes,
            AllTests,
            e::E,
            e::f::F
        ),
    )
    .unwrap();

    let all_tests = AllTests {
        a: ContainerTypes {
            e_a: EnumContainer::A,
            e_b: EnumContainer::B(123),
            e_c: EnumContainer::C(
                123,
                StructContainer {
                    a: 123,
                    b: 123,
                    c: 123,
                    d: 123,
                },
            ),
            e_d: EnumContainer::D {
                a: 123,
                b: StructContainer {
                    a: 123,
                    b: 123,
                    c: 123,
                    d: 123,
                },
            },
            s: StructContainer {
                a: 123,
                b: 123,
                c: 123,
                d: 123,
            },
            t: TupleContainer(
                123,
                StructContainer {
                    a: 123,
                    b: 123,
                    c: 123,
                    d: 123,
                },
                EnumContainer::A,
            ),
            u: UnionContainer,
        },
        b: PrimitiveTypes {
            u8: u8::MAX,
            u16: u16::MAX,
            u32: u32::MAX,
            u64: u64::MAX,
            u128: u128::MAX,
            usize: usize::MAX,
            i8_max: i8::MAX,
            i8_min: i8::MIN,
            i16_max: i16::MAX,
            i16_min: i16::MIN,
            i32_max: i32::MAX,
            i32_min: i32::MIN,
            i64_max: i64::MAX,
            i64_min: i64::MIN,
            i128_max: i128::MAX,
            i128_min: i128::MIN,
            isize_max: isize::MAX,
            isize_min: isize::MIN,
            f32: 123.123,
            f64: 123.123,
            bool_true: true,
            bool_false: false,
            // TODO: char is not supported yet
            // char: 'a',
            none_zero: NonZero::new(123).unwrap(),
        },
        c: CompoundTypes {
            static_byte_slice: &[123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
            static_str: "Hello",
            array: [123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
            range: 10..20,
            option_some: Some(123),
            option_none: None,
            tuple: (
                123,
                StructContainer {
                    a: 123,
                    b: 123,
                    c: 123,
                    d: 123,
                },
                EnumContainer::A,
                TupleContainer(
                    123,
                    StructContainer {
                        a: 123,
                        b: 123,
                        c: 123,
                        d: 123,
                    },
                    EnumContainer::A,
                ),
            ),
        },
        d: AllocTypes {
            a: vec![123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
            b: "Hello".into(),
            c: {
                let mut map = HashMap::new();
                map.insert(123, 123);
                map
            },
        },
        e: HeaplessTypes {
            a: heapless::Vec::from_slice(&[123, 123, 123, 123, 123, 123, 123, 123, 123, 123])
                .unwrap(),
            b: heapless::String::from_str("Hello").unwrap(),
            c: {
                let mut map = heapless::LinearMap::new();
                map.insert(123, 123).unwrap();
                map
            },
        },
        f: e::E(123, e::f::F(123)),
    };

    let postcard_bytes = postcard::to_vec::<_, 300>(&all_tests).expect("Failed to serialize");
    let mut file =
        std::fs::File::create(std::env::current_dir().unwrap().join("serialized.bytes")).unwrap();
    file.write_all(postcard_bytes.as_slice()).unwrap();
}
