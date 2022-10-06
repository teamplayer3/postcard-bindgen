use genco::{lang::js::Tokens, quote};

use crate::utils::StrExt;

pub fn gen_check_func(obj_name: impl AsRef<str>) -> Tokens {
    let obj_name = obj_name.as_ref();

    quote!(const is_$(obj_name.to_obj_identifier()) = (v) => (typeof v === "object" && Object.keys(v).length === 0 ))
}
