use genco::{lang::js::Tokens, quote};

pub fn gen_check_func() -> Tokens {
    quote!(typeof v === "object" && Object.keys(v).length === 0)
}
