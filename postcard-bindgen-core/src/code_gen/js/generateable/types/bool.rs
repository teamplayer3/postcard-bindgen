use genco::quote;

use crate::{
    code_gen::js::{FieldAccessor, VariablePath},
    type_info::BoolMeta,
};

use super::JsTypeGenerateable;

impl JsTypeGenerateable for BoolMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> genco::prelude::js::Tokens {
        quote!(s.serialize_bool($variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> genco::prelude::js::Tokens {
        quote!($(field_accessor)d.deserialize_bool())
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> genco::prelude::js::Tokens {
        quote!(typeof $variable_path === "boolean")
    }

    fn gen_ts_type(&self) -> genco::prelude::js::Tokens {
        quote!(boolean)
    }
}

pub fn bool_to_js_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
