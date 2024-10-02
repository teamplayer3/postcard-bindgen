use alloc::{boxed::Box, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
    Optional(OptionalMeta),
    Range(RangeMeta),
    Map(MapMeta),
    Tuple(TupleMeta),
}

impl AsRef<JsType> for JsType {
    fn as_ref(&self) -> &JsType {
        self
    }
}

pub fn bool_to_js_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapMeta {
    pub(crate) key_type: Box<JsType>,
    pub(crate) value_type: Box<JsType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeMeta {
    pub(crate) bounds_type: Box<JsType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalMeta {
    pub(crate) inner: Box<JsType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberMeta {
    Integer { bytes: usize, signed: bool },
    FloatingPoint { bytes: usize },
}

#[cfg(feature = "generating")]
mod int_byte_len {
    use super::NumberMeta;

    const U8_BYTES_CONST: &str = "U8_BYTES";
    const U16_BYTES_CONST: &str = "U16_BYTES";
    const U32_BYTES_CONST: &str = "U32_BYTES";
    const U64_BYTES_CONST: &str = "U64_BYTES";
    const U128_BYTES_CONST: &str = "U128_BYTES";
    // const USIZE_BYTES_CONST: &str = "USIZE_BYTES";

    impl NumberMeta {
        pub(crate) fn as_byte_js_string(&self) -> &'static str {
            let bytes = match self {
                NumberMeta::Integer { bytes, .. } => bytes,
                NumberMeta::FloatingPoint { bytes } => bytes,
            };
            match bytes {
                1 => U8_BYTES_CONST,
                2 => U16_BYTES_CONST,
                4 => U32_BYTES_CONST,
                8 => U64_BYTES_CONST,
                16 => U128_BYTES_CONST,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayMeta {
    // Boxed to avoid infinite recursion
    pub(crate) items_type: Box<JsType>,
    pub(crate) length: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringMeta {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMeta {
    pub name: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleMeta {
    pub(crate) items_types: Vec<JsType>,
}

pub trait GenJsBinding {
    fn get_type() -> JsType;
}

impl<T: GenJsBinding> GenJsBinding for &mut T {
    fn get_type() -> JsType {
        T::get_type()
    }
}

macro_rules! impl_gen_js_binding_numbers_ints {
    ($ty:ty, $bytes:expr, $signed:ident) => {
        impl GenJsBinding for $ty {
            fn get_type() -> JsType {
                JsType::Number(NumberMeta::Integer {
                    bytes: $bytes,
                    signed: $signed,
                })
            }
        }
    };
}

macro_rules! impl_gen_js_binding_numbers_floats {
    ($ty:ty, $bytes:expr) => {
        impl GenJsBinding for $ty {
            fn get_type() -> JsType {
                JsType::Number(NumberMeta::FloatingPoint { bytes: $bytes })
            }
        }
    };
}

impl_gen_js_binding_numbers_ints!(u8, 1, false);
impl_gen_js_binding_numbers_ints!(u16, 2, false);
impl_gen_js_binding_numbers_ints!(u32, 4, false);
impl_gen_js_binding_numbers_ints!(u64, 8, false);
impl_gen_js_binding_numbers_ints!(u128, 16, false);
// TODO check for operating system
impl_gen_js_binding_numbers_ints!(usize, 4, false);

impl_gen_js_binding_numbers_ints!(i8, 1, true);
impl_gen_js_binding_numbers_ints!(i16, 2, true);
impl_gen_js_binding_numbers_ints!(i32, 4, true);
impl_gen_js_binding_numbers_ints!(i64, 8, true);
impl_gen_js_binding_numbers_ints!(i128, 16, true);
// TODO check for operating system
impl_gen_js_binding_numbers_ints!(isize, 4, true);

impl_gen_js_binding_numbers_floats!(f32, 4);
impl_gen_js_binding_numbers_floats!(f64, 8);

impl<T: GenJsBinding> GenJsBinding for Option<T> {
    fn get_type() -> JsType {
        JsType::Optional(OptionalMeta {
            inner: Box::new(T::get_type()),
        })
    }
}

impl<'a, T: GenJsBinding> GenJsBinding for &'a [T] {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
        })
    }
}

impl<T: GenJsBinding> GenJsBinding for [T] {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
        })
    }
}

impl<T: GenJsBinding, const S: usize> GenJsBinding for [T; S] {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: Some(S),
        })
    }
}

impl<'a> GenJsBinding for &'a str {
    fn get_type() -> JsType {
        JsType::String(StringMeta {})
    }
}

impl<T: GenJsBinding> GenJsBinding for core::ops::Range<T> {
    fn get_type() -> JsType {
        JsType::Range(RangeMeta {
            bounds_type: Box::new(T::get_type()),
        })
    }
}

#[cfg(feature = "alloc")]
impl<K: GenJsBinding, V: GenJsBinding> GenJsBinding for alloc::collections::BTreeMap<K, V> {
    fn get_type() -> JsType {
        JsType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
        })
    }
}

#[cfg(feature = "alloc")]
impl GenJsBinding for alloc::string::String {
    fn get_type() -> JsType {
        JsType::String(StringMeta {})
    }
}

#[cfg(feature = "alloc")]
impl<T: GenJsBinding> GenJsBinding for alloc::vec::Vec<T> {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
        })
    }
}

#[cfg(feature = "std")]
impl<K: GenJsBinding, V: GenJsBinding> GenJsBinding for std::collections::HashMap<K, V> {
    fn get_type() -> JsType {
        JsType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
        })
    }
}

#[cfg(feature = "heapless")]
impl<T: GenJsBinding, const N: usize> GenJsBinding for heapless::Vec<T, N> {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
}

#[cfg(feature = "heapless")]
impl<const N: usize> GenJsBinding for heapless::String<N> {
    fn get_type() -> JsType {
        JsType::String(StringMeta {})
    }
}

#[cfg(feature = "heapless")]
impl<K: GenJsBinding, V: GenJsBinding, const N: usize> GenJsBinding
    for heapless::LinearMap<K, V, N>
{
    fn get_type() -> JsType {
        JsType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
        })
    }
}
