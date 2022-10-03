#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
}

impl ToString for JsType {
    fn to_string(&self) -> String {
        match self {
            JsType::Array(_) => "array".into(),
            JsType::Number(_) => "number".into(),
            JsType::Object(_) => "object".into(),
            JsType::String(_) => "string".into(),
        }
    }
}

impl JsType {
    pub(crate) fn as_js_func_args(&self) -> Vec<&'static str> {
        match self {
            JsType::Number(m) => {
                vec![m.as_byte_js_string(), bool_to_js_bool(m.signed)]
            }
            JsType::Array(ArrayMeta { items_type: _ }) => vec![],
            JsType::String(_) => todo!(),
            JsType::Object(_m) => todo!(),
        }
    }

    pub(crate) fn as_func_name(&self) -> &'static str {
        match self {
            JsType::Number(_) => "number",
            JsType::Array(_) => "array",
            JsType::String(_) => "string",
            JsType::Object(_) => unimplemented!(),
        }
    }
}

impl AsRef<JsType> for JsType {
    fn as_ref(&self) -> &JsType {
        self
    }
}

fn bool_to_js_bool(value: bool) -> &'static str {
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
    pub name: String,
}

pub trait GenJsBinding {
    fn get_type() -> JsType;
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

impl GenJsBinding for std::string::String {
    fn get_type() -> JsType {
        JsType::String(StringMeta {})
    }
}

impl<T: GenJsBinding> GenJsBinding for std::vec::Vec<T> {
    fn get_type() -> JsType {
        JsType::Array(ArrayMeta {
            items_type: Box::new(T::get_type()),
        })
    }
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
