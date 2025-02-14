use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::js::{FieldAccessor, VariablePath},
    type_info::NumberMeta,
};

use super::{bool::bool_to_js_bool, JsTypeGenerateable};

impl JsTypeGenerateable for NumberMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let byte_amount_str = self.as_byte_string();
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!(s.serialize_number_float($byte_amount_str, $variable_path))
            }
            NumberMeta::Integer { signed, .. } => {
                let signed = bool_to_js_bool(*signed);
                quote!(s.serialize_number($byte_amount_str, $signed, $variable_path))
            }
        }
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let byte_amount_str = self.as_byte_string();
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!($(field_accessor)d.deserialize_number_float($byte_amount_str))
            }
            NumberMeta::Integer { signed, .. } => {
                let signed = bool_to_js_bool(*signed);
                quote!($(field_accessor)d.deserialize_number($byte_amount_str, $signed))
            }
        }
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let byte_amount_str = self.as_byte_string();
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!(typeof $(variable_path.to_owned()) === "number" && Number.isFinite($(variable_path.to_owned())))
            }
            NumberMeta::Integer {
                signed, zero_able, ..
            } => {
                let signed = bool_to_js_bool(*signed);
                let zero_able = bool_to_js_bool(*zero_able);
                quote!(check_integer_type($variable_path, $byte_amount_str, $signed, $zero_able))
            }
        }
    }

    fn gen_ts_type(&self) -> Tokens {
        match self {
            NumberMeta::FloatingPoint { bytes } => {
                let bits = match bytes {
                    4 => "32",
                    8 => "64",
                    _ => unreachable!(),
                };
                quote!(f$bits)
            }
            NumberMeta::Integer {
                bytes,
                signed,
                zero_able,
            } => {
                let prefix = if *signed { "i" } else { "u" };
                let bits = match bytes {
                    1 => "8",
                    2 => "16",
                    4 => "32",
                    8 => "64",
                    16 => "128",
                    _ => unreachable!(),
                };
                if *zero_able {
                    quote!($prefix$bits)
                } else {
                    quote!(NonZero$(prefix.to_uppercase())$bits)
                }
            }
        }
    }
}
