use convert_case::{Case, Casing};
use genco::{lang::js::Tokens, quote, quote_in};

use crate::JsTyping;

pub fn gen_serialize_func(defines: &[JsTyping]) -> Tokens {
    quote!(
        module.exports.serialize = (type, value) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const s = new Serializer()
            switch (type) {
                $(gen_ser_cases(defines))
            }
            return s.finish()
        }
    )
}

fn gen_ser_cases(defines: &[JsTyping]) -> Tokens {
    let mut tokens = Tokens::new();
    defines.iter().for_each(|define| {
        gen_ser_case(&mut tokens, define);
        tokens.append(";");
    });
    tokens
}

fn gen_ser_case(tokens: &mut Tokens, define: &JsTyping) {
    let case = format!("\"{}\"", define.type_ident);
    quote_in! {*tokens =>
        case $case: serialize_$(define.type_ident.to_case(Case::Snake).to_uppercase())(s, value); break
    }
}
