use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::{generateable::VariablePath, JS_OBJECT_VARIABLE},
    type_info::ArrayMeta,
};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for ArrayMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let inner_type_accessor = self.items_type.gen_ser_accessor(VariablePath::default());
        quote!(s.serialize_array((s, $JS_OBJECT_VARIABLE) => $inner_type_accessor, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let inner_type_accessor = self.items_type.gen_des_accessor(des::FieldAccessor::Array);
        quote!($(field_accessor)d.deserialize_array(() => $inner_type_accessor))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        quote!(Array.isArray($variable_path))
    }

    fn gen_ts_type(&self) -> Tokens {
        quote!($(self.items_type.gen_ts_type())[])
    }
}
