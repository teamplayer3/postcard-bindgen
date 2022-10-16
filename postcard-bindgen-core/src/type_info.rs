#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
    Optional(Box<JsType>),
}

impl ToString for JsType {
    fn to_string(&self) -> String {
        match self {
            JsType::Array(_) => "array".into(),
            JsType::Number(_) => "number".into(),
            JsType::Object(_) => "object".into(),
            JsType::String(_) => "string".into(),
            JsType::Optional(_) => "optional".into(),
        }
    }
}

impl JsType {
    pub(crate) fn as_func_name(&self) -> &'static str {
        match self {
            JsType::Number(_) => "number",
            JsType::Array(_) => "array",
            JsType::String(_) => "string",
            JsType::Object(_) => "object",
            JsType::Optional(_) => "optional",
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberMeta {
    pub(crate) bytes: usize,
    pub(crate) signed: bool,
}

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

    pub(crate) fn as_ts_type(&self) -> String {
        let prefix = if self.signed { "i" } else { "u" };
        let bits = match self.bytes {
            1 => "8",
            2 => "16",
            4 => "32",
            8 => "64",
            16 => "128",
            _ => unreachable!(),
        };
        let mut out = String::from(prefix);
        out.push_str(bits);
        out
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
        JsType::Optional(Box::new(T::get_type()))
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
