use genco::quote;

use crate::{
    code_gen::js::{Tokens, JS_OBJECT_VARIABLE},
    registry::UnitStructType,
};

use super::{des, ts, BindingTypeGenerateable};

impl BindingTypeGenerateable for UnitStructType {
    fn gen_ser_body(&self) -> Tokens {
        quote!()
    }

    fn gen_des_body(&self) -> Tokens {
        let body = des::gen_accessors_fields([]);
        quote!(return $body;)
    }

    fn gen_ty_check_body(&self) -> Tokens {
        quote!(return typeof $JS_OBJECT_VARIABLE === "object" && Object.keys($JS_OBJECT_VARIABLE).length === 0)
    }

    fn gen_ts_typings_body(&self) -> Tokens {
        ts::gen_typings_fields([])
    }
}
