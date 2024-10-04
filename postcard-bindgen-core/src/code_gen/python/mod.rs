
use genco::{lang::python::Tokens, quote};

use crate::registry::BindingType;

use self::ser_des::gen_ser_des_classes;

mod ser_des;
mod generateable;
pub mod type_checking;



pub fn generate(tys: impl AsRef<[BindingType]>) -> Tokens {
    // let ser_des_body = gen_ser_des_functions(&tys);
    // let type_checks = gen_type_checkings(&tys);
    quote!(
        $(gen_ser_des_classes())
        // $ser_des_body
        // $type_checks
        // $(gen_serialize_func(&tys))
        // $(gen_deserialize_func(tys))
    )
}