use convert_case::{Case, Casing};
use genco::{lang::js::Tokens, quote, quote_in};

use crate::JsTyping;

pub fn gen_deserialize_func(defines: &Vec<JsTyping>) -> Tokens {
    quote!(
        module.exports.deserialize = (type, bytes) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const d = new Deserializer(bytes)
            switch (type) {
                $(gen_des_cases(defines))
            }
        }
    )
}

fn gen_des_cases(defines: &Vec<JsTyping>) -> Tokens {
    let mut tokens = Tokens::new();
    defines.iter().for_each(|define| {
        gen_des_case(&mut tokens, define);
        tokens.append(";");
    });
    tokens
}

fn gen_des_case(tokens: &mut Tokens, define: &JsTyping) {
    let case = format!("\"{}\"", define.type_ident);
    quote_in! {*tokens =>
        case $case: return deserialize_$(define.type_ident.to_case(Case::Snake).to_uppercase())(d)
    }
}
