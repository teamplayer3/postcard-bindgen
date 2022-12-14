use alloc::boxed::Box;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
    Optional(OptionalMeta),
    Range(RangeMeta),
    Map(MapMeta),
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
pub struct NumberMeta {
    pub(crate) bytes: usize,
    pub(crate) signed: bool,
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
            match self.bytes {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringMeta {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMeta {
    pub name: &'static str,
}

pub trait GenJsBinding {
    fn get_type() -> JsType;
}

impl<T: GenJsBinding> GenJsBinding for &mut T {
    fn get_type() -> JsType {
        T::get_type()
    }
}

macro_rules! impl_gen_js_binding_numbers {
    ($ty:ty, $bytes:expr, $signed:ident) => {
        impl GenJsBinding for $ty {
            fn get_type() -> JsType {
                JsType::Number(NumberMeta {
                    bytes: $bytes,
                    signed: $signed,
                })
            }
        }
    };
}

impl_gen_js_binding_numbers!(u8, 1, false);
impl_gen_js_binding_numbers!(u16, 2, false);
impl_gen_js_binding_numbers!(u32, 4, false);
impl_gen_js_binding_numbers!(u64, 8, false);
impl_gen_js_binding_numbers!(u128, 16, false);
// TODO check for operating system
impl_gen_js_binding_numbers!(usize, 4, false);

impl_gen_js_binding_numbers!(i8, 1, true);
impl_gen_js_binding_numbers!(i16, 2, true);
impl_gen_js_binding_numbers!(i32, 4, true);
impl_gen_js_binding_numbers!(i64, 8, true);
impl_gen_js_binding_numbers!(i128, 16, true);
// TODO check for operating system
impl_gen_js_binding_numbers!(isize, 4, true);

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
        })
    }
}

impl<T: GenJsBinding> GenJsBinding for [T] {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
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
