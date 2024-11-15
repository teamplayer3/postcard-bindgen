use genco::{lang::js::Tokens, quote};

use crate::{
    code_gen::utils::{ContainerIdentifierBuilder, TokensIterExt},
    registry::Container,
};

use super::{generateable::container::BindingTypeGenerateable, JS_OBJECT_VARIABLE};

pub fn gen_type_checkings(bindings: impl Iterator<Item = Container>) -> Tokens {
    let body = bindings.map(gen_type_check).join_with_line_breaks();
    quote! {
        const check_bounds = (n_bytes, signed, value) => { const max = BigInt(2 ** (n_bytes * BITS_PER_BYTE)), value_b = BigInt(value); if (signed) { const bounds = max / 2n; return value_b >= -bounds && value_b < bounds } else { return value_b < max && value_b >= 0 } }

        $body
    }
}

pub fn gen_type_check(container: Container) -> Tokens {
    let container_ident = ContainerIdentifierBuilder::from(&container).build();
    let body = container.r#type.gen_ty_check_body();
    quote!(const is_$container_ident = ($JS_OBJECT_VARIABLE) => ($body))
}
