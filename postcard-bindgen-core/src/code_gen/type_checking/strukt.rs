use genco::{lang::js::Tokens, quote};

use crate::{registry::StructField, utils::StrExt};

use super::{gen_object_checks, InnerTypeAccess};

pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
    let obj_name = obj_name.as_ref();
    let body = gen_object_checks(fields, InnerTypeAccess::Direct);
    quote!(const is_$(obj_name.to_obj_identifier()) = (v) => ($body))
}
