use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::generateable::VariablePath,
    type_info::{bool_to_js_bool, NumberMeta},
};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for NumberMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let byte_amount_str = self.as_byte_js_string();
        let signed = bool_to_js_bool(self.signed);
        quote!(s.serialize_number($byte_amount_str, $signed, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let byte_amount_str = self.as_byte_js_string();
        let signed = bool_to_js_bool(self.signed);
        quote!($(field_accessor)d.deserialize_number($byte_amount_str, $signed))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        quote!(typeof $variable_path === "number")
    }

    fn gen_ts_type(&self) -> Tokens {
        let prefix = if self.signed { "i" } else { "u" };
        let bits = match self.bytes {
            1 => "8",
            2 => "16",
            4 => "32",
            8 => "64",
            16 => "128",
            _ => unreachable!(),
        };
        quote!($prefix$bits)
    }
}
