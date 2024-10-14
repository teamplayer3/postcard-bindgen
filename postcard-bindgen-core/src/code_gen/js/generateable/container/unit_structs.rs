use genco::quote;

use crate::{
    code_gen::{
        js::{Tokens, VariablePath, JS_OBJECT_VARIABLE},
        utils::wrapped_brackets,
    },
    registry::UnitStructType,
};

use super::{des, ser, ts, BindingTypeGenerateable};

impl BindingTypeGenerateable for UnitStructType {
    fn gen_ser_body(&self) -> Tokens {
        ser::gen_accessors_fields([], VariablePath::default())
    }

    fn gen_des_body(&self) -> Tokens {
        wrapped_brackets(des::gen_accessors_fields([]))
    }

    fn gen_ty_check_body(&self) -> Tokens {
        quote!(typeof $JS_OBJECT_VARIABLE === "object" && Object.keys($JS_OBJECT_VARIABLE).length === 0)
    }

    fn gen_ts_typings_body(&self) -> Tokens {
        ts::gen_typings_fields([])
    }
}
