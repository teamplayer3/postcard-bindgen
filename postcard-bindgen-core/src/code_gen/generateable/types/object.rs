use genco::{prelude::js::Tokens, quote};

use crate::{code_gen::generateable::VariablePath, type_info::ObjectMeta, utils::StrExt};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for ObjectMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let obj_ident = self.name.to_obj_identifier();
        quote!(serialize_$obj_ident(s, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let obj_ident = self.name.to_obj_identifier();
        quote!($(field_accessor)deserialize_$obj_ident(d))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let obj_ident = self.name.to_obj_identifier();
        quote!(is_$obj_ident($variable_path))
    }

    fn gen_ts_type(&self) -> Tokens {
        quote!($(self.name))
    }
}
