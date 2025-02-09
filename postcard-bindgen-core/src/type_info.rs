use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

use alloc::{boxed::Box, vec, vec::Vec};

use crate::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
    Optional(OptionalMeta),
    Range(RangeMeta),
    Map(MapMeta),
    Tuple(TupleMeta),
    Bool(BoolMeta),
}

impl ValueType {
    pub fn flatten_paths(&mut self) {
        match self {
            ValueType::Object(meta) => {
                meta.path.flatten();
            }
            ValueType::Array(meta) => {
                meta.items_type.flatten_paths();
            }
            ValueType::Optional(meta) => {
                meta.inner.flatten_paths();
            }
            ValueType::Map(meta) => {
                meta.key_type.flatten_paths();
                meta.value_type.flatten_paths();
            }
            ValueType::Tuple(meta) => {
                for item in meta.items_types.iter_mut() {
                    item.flatten_paths();
                }
            }
            _ => {}
        }
    }
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
    pub(crate) max_length: Option<usize>,
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
    Integer {
        bytes: usize,
        signed: bool,
        zero_able: bool,
    },
    FloatingPoint {
        bytes: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayMeta {
    // Boxed to avoid infinite recursion
    pub(crate) items_type: Box<ValueType>,
    pub(crate) length: Option<usize>,
    pub(crate) max_length: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringMeta {
    pub(crate) max_length: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMeta {
    pub name: &'static str,
    pub path: Path<'static, 'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleMeta {
    pub(crate) items_types: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolMeta;

pub trait GenBinding {
    fn get_type() -> ValueType;
}

impl<T: GenBinding> GenBinding for &mut T {
    fn get_type() -> ValueType {
        T::get_type()
    }
}

macro_rules! impl_gen_js_binding_numbers_ints {
    ($($ty:ty: $bytes:expr, $signed:ident, $zero_able:ident);*) => {
        $(
            impl GenBinding for $ty {
                fn get_type() -> ValueType {
                    ValueType::Number(NumberMeta::Integer {
                        bytes: $bytes,
                        signed: $signed,
                        zero_able: $zero_able,
                    })
                }
            }
        )*
    };
}

macro_rules! impl_gen_js_binding_numbers_floats {
    ($($ty:ty: $bytes:expr);*) => {
        $(
            impl GenBinding for $ty {
                fn get_type() -> ValueType {
                    ValueType::Number(NumberMeta::FloatingPoint { bytes: $bytes })
                }
            }
        )*
    };
}

impl_gen_js_binding_numbers_ints![
    u8: 1, false, true;
    u16: 2, false, true;
    u32: 4, false, true;
    u64: 8, false, true;
    u128: 1, false, true;

    i8: 1, true, true;
    i16: 2, true, true;
    i32: 4, true, true;
    i64: 8, true, true;
    i128: 16, true, true;

    usize: 4, false, true;
    isize: 4, true, true;

    NonZeroU8: 1, false, false;
    NonZeroU16: 2, false, false;
    NonZeroU32: 4, false, false;
    NonZeroU64: 8, false, false;
    NonZeroU128: 16, false, false;

    NonZeroI8: 1, true, false;
    NonZeroI16: 2, true, false;
    NonZeroI32: 4, true, false;
    NonZeroI64: 8, true, false;
    NonZeroI128: 16, true, false
];

impl_gen_js_binding_numbers_floats![
    f32: 4;
    f64: 8
];

impl<T: GenBinding> GenBinding for Option<T> {
    fn get_type() -> ValueType {
        ValueType::Optional(OptionalMeta {
            inner: Box::new(T::get_type()),
        })
    }
}

impl<T: GenBinding> GenBinding for &[T] {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
            max_length: None,
        })
    }
}

impl<T: GenBinding> GenBinding for [T] {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
            max_length: None,
        })
    }
}

impl<T: GenBinding, const S: usize> GenBinding for [T; S] {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: Some(S),
            max_length: Some(S),
        })
    }
}

impl GenBinding for &str {
    fn get_type() -> ValueType {
        ValueType::String(StringMeta { max_length: None })
    }
}

impl<T: GenBinding> GenBinding for core::ops::Range<T> {
    fn get_type() -> ValueType {
        ValueType::Range(RangeMeta {
            bounds_type: Box::new(T::get_type()),
        })
    }
}

macro_rules! tuple_impls {
    ($($($name:ident)+),+) => {
        $(
            impl<$($name: GenBinding),+> GenBinding for ($($name),+) {
                fn get_type() -> ValueType {
                    ValueType::Tuple(TupleMeta {
                        items_types: vec![$($name::get_type()),+],
                    })
                }
            }
        )+
    };
}

impl<T: GenBinding> GenBinding for (T,) {
    fn get_type() -> ValueType {
        ValueType::Tuple(TupleMeta {
            items_types: vec![T::get_type()],
        })
    }
}

tuple_impls! {
    T0 T1,
    T0 T1 T2,
    T0 T1 T2 T3,
    T0 T1 T2 T3 T4,
    T0 T1 T2 T3 T4 T5,
    T0 T1 T2 T3 T4 T5 T6,
    T0 T1 T2 T3 T4 T5 T6 T7,
    T0 T1 T2 T3 T4 T5 T6 T7 T8,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14
}

impl GenBinding for bool {
    fn get_type() -> ValueType {
        ValueType::Bool(BoolMeta)
    }
}

#[cfg(feature = "alloc")]
impl<K: GenBinding, V: GenBinding> GenBinding for alloc::collections::BTreeMap<K, V> {
    fn get_type() -> ValueType {
        ValueType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
            max_length: None,
        })
    }
}

#[cfg(feature = "alloc")]
impl GenBinding for alloc::string::String {
    fn get_type() -> ValueType {
        ValueType::String(StringMeta { max_length: None })
    }
}

#[cfg(feature = "alloc")]
impl<T: GenBinding> GenBinding for alloc::vec::Vec<T> {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
            max_length: None,
        })
    }
}

#[cfg(feature = "alloc")]
impl<T: GenBinding> GenBinding for alloc::rc::Rc<T> {
    fn get_type() -> ValueType {
        T::get_type()
    }
}

#[cfg(feature = "alloc")]
impl<T: GenBinding> GenBinding for alloc::sync::Arc<T> {
    fn get_type() -> ValueType {
        T::get_type()
    }
}

#[cfg(feature = "std")]
impl<K: GenBinding, V: GenBinding> GenBinding for std::collections::HashMap<K, V> {
    fn get_type() -> ValueType {
        ValueType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
            max_length: None,
        })
    }
}

#[cfg(feature = "std")]
impl<T: GenBinding> GenBinding for std::sync::RwLock<T> {
    fn get_type() -> ValueType {
        T::get_type()
    }
}

#[cfg(feature = "heapless")]
impl<T: GenBinding, const N: usize> GenBinding for heapless::Vec<T, N> {
    fn get_type() -> ValueType {
        ValueType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
            length: None,
            max_length: Some(N),
        })
    }
}

#[cfg(feature = "heapless")]
impl<const N: usize> GenBinding for heapless::String<N> {
    fn get_type() -> ValueType {
        ValueType::String(StringMeta {
            max_length: Some(N),
        })
    }
}

#[cfg(feature = "heapless")]
impl<K: GenBinding, V: GenBinding, const N: usize> GenBinding for heapless::LinearMap<K, V, N> {
    fn get_type() -> ValueType {
        ValueType::Map(MapMeta {
            key_type: Box::new(K::get_type()),
            value_type: Box::new(V::get_type()),
            max_length: Some(N),
        })
    }
}
