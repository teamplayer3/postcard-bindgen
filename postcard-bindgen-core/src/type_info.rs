use alloc::boxed::Box;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
    Optional(OptionalMeta),
    Range(RangeMeta),
    Map(MapMeta),
}

impl AsRef<ValueType> for ValueType {
    fn as_ref(&self) -> &ValueType {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapMeta {
    pub(crate) key_type: Box<ValueType>,
    pub(crate) value_type: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeMeta {
    pub(crate) bounds_type: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalMeta {
    pub(crate) inner: Box<ValueType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberMeta {
    Integer { bytes: usize, signed: bool },
    FloatingPoint { bytes: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayMeta {
    // Boxed to avoid infinite recursion
    pub(crate) items_type: Box<ValueType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringMeta {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMeta {
    pub name: &'static str,
}

pub trait GenJsBinding {
    fn get_type() -> ValueType;
}

impl<T: GenJsBinding> GenJsBinding for &mut T {
    fn get_type() -> ValueType {
        T::get_type()
    }
}

macro_rules! impl_gen_js_binding_numbers_ints {
    ($ty:ty, $bytes:expr, $signed:ident) => {
        impl GenJsBinding for $ty {
            fn get_type() -> ValueType {
                ValueType::Number(NumberMeta::Integer {
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
            fn get_type() -> ValueType {
                ValueType::Number(NumberMeta::FloatingPoint { bytes: $bytes })
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
    fn get_type() -> ValueType {
        ValueType::Optional(OptionalMeta {
            inner: Box::new(T::get_type()),
        })
    }
}

impl<'a, T: GenJsBinding> GenJsBinding for &'a [T] {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
}

impl<T: GenJsBinding> GenJsBinding for [T] {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
}

impl<T: GenJsBinding, const S: usize> GenJsBinding for [T; S] {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
}

impl<'a> GenJsBinding for &'a str {
    fn get_type() -> ValueType {
        ValueType::String(StringMeta {})
    }
}

impl<T: GenJsBinding> GenJsBinding for core::ops::Range<T> {
    fn get_type() -> ValueType {
        ValueType::Range(RangeMeta {
            bounds_type: Box::new(T::get_type()),
        })
    }
}

#[cfg(feature = "alloc")]
impl<K: GenJsBinding, V: GenJsBinding> GenJsBinding for alloc::collections::BTreeMap<K, V> {
    fn get_type() -> ValueType {
        ValueType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
        })
    }
}

#[cfg(feature = "alloc")]
impl GenJsBinding for alloc::string::String {
    fn get_type() -> ValueType {
        ValueType::String(StringMeta {})
    }
}

#[cfg(feature = "alloc")]
impl<T: GenJsBinding> GenJsBinding for alloc::vec::Vec<T> {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
}

#[cfg(feature = "std")]
impl<K: GenJsBinding, V: GenJsBinding> GenJsBinding for std::collections::HashMap<K, V> {
    fn get_type() -> ValueType {
        ValueType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
        })
    }
}

#[cfg(feature = "heapless")]
impl<T: GenJsBinding, const N: usize> GenJsBinding for heapless::Vec<T, N> {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
}

#[cfg(feature = "heapless")]
impl<const N: usize> GenJsBinding for heapless::String<N> {
    fn get_type() -> ValueType {
        ValueType::String(StringMeta {})
    }
}

#[cfg(feature = "heapless")]
impl<K: GenJsBinding, V: GenJsBinding, const N: usize> GenJsBinding
    for heapless::LinearMap<K, V, N>
{
    fn get_type() -> ValueType {
        ValueType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
        })
    }
}
