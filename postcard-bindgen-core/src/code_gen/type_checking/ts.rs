use genco::{prelude::js::Tokens, quote, tokens::quoted};

use crate::{
    code_gen::utils::{colon_chain, divider_chain},
    registry::BindingType,
};

pub fn gen_ts_typings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    quote!(
        $(gen_type_decl(&bindings))
        $(gen_value_type_decl(bindings))
        $(gen_ser_des_decls())
    )
}

fn gen_type_decl(bindings: impl AsRef<[BindingType]>) -> Tokens {
    let type_cases = divider_chain(
        bindings
            .as_ref()
            .iter()
            .map(|b| quote!($(quoted(b.inner_name())))),
    );
    quote!(export type Type = $type_cases)
}

fn gen_value_type_decl(bindings: impl AsRef<[BindingType]>) -> Tokens {
    let if_cases = colon_chain(
        bindings
            .as_ref()
            .iter()
            .map(|b| quote!(T extends $(quoted(b.inner_name())) ? $(b.inner_name()))),
    );
    quote!(declare type ValueType<T extends Type> = $if_cases : void)
}

fn gen_ser_des_decls() -> Tokens {
    quote!(
        export function serialize<T extends Type>(type: T, value: ValueType<T>): u8[]
        export function deserialize<T extends Type>(type: T, bytes: u8[]): ValueType<T>
    )
}
