use genco::{lang::js::Tokens, quote};

use crate::{
    code_gen::{generateable::VariablePath, JS_OBJECT_VARIABLE},
    type_info::TupleMeta,
};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for TupleMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        unimplemented!()
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        unimplemented!()
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        quote!(Array.isArray($variable_path))
    }

    fn gen_ts_type(&self) -> Tokens {
        // quote!([$(self.items_types.gen_ts_type()),*])
        unimplemented!()
    }
}
