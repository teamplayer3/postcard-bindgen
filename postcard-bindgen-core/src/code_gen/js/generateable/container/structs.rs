use crate::{
    code_gen::{
        js::{Tokens, VariablePath},
        utils::wrapped_brackets,
    },
    registry::StructType,
};

use super::{des, ser, ts, ty_check, BindingTypeGenerateable};

impl BindingTypeGenerateable for StructType {
    fn gen_ser_body(&self) -> Tokens {
        ser::gen_accessors_fields(&self.fields, VariablePath::default())
    }

    fn gen_des_body(&self) -> Tokens {
        wrapped_brackets(des::gen_accessors_fields(&self.fields))
    }

    fn gen_ty_check_body(&self) -> Tokens {
        ty_check::gen_object_checks(&self.fields, VariablePath::default())
    }

    fn gen_ts_typings_body(&self) -> Tokens {
        ts::gen_typings_fields(&self.fields)
    }
}
