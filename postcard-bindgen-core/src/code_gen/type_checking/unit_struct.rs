use genco::{lang::js::Tokens, quote};

use crate::code_gen::JS_OBJECT_VARIABLE;

pub fn gen_check_func() -> Tokens {
    quote!(typeof $JS_OBJECT_VARIABLE === "object" && Object.keys($JS_OBJECT_VARIABLE).length === 0)
}
