pub enum JsType {
    Number(NumberMeta),
    Array(ArrayMeta),
    String(StringMeta),
    Object(ObjectMeta),
}

pub struct NumberMeta {
    pub(crate) bytes: usize,
    pub(crate) signed: bool,
}

pub struct ArrayMeta {
    // Boxed to avoid infinite recursion
    pub(crate) items_type: Box<JsType>,
}

pub struct StringMeta {}

pub struct ObjectMeta {
    name: String,
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
