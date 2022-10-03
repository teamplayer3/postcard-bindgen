use genco::{lang::js::Tokens, quote};

use crate::StrExt;

pub fn gen_check_func(obj_name: impl AsRef<str>) -> Tokens {
    let obj_name = obj_name.as_ref();

    quote! {
        const is_$(obj_name.to_obj_identifier()) = (v) => {
            return typeof v === "object" && Object.keys(v).length === 0
        }
    }
}
