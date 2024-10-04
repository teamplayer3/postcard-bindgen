use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::js::{FieldAccessor, VariablePath},
    type_info::StringMeta,
};

use super::JsTypeGenerateable;

impl JsTypeGenerateable for StringMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        quote!(s.serialize_string($variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        quote!($(field_accessor)d.deserialize_string())
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        quote!(typeof $variable_path === "string")
    }

    fn gen_ts_type(&self) -> Tokens {
        quote!(string)
    }
}
