use genco::{
    lang::{js::Tokens, JavaScript},
    quote,
    tokens::FormatInto,
};

use crate::{
    code_gen::{
        function::Function,
        utils::{break_long_logical_lines, ContainerIdentifierBuilder, TokensIterExt},
    },
    function_args,
    registry::Container,
};

use super::{generateable::container::BindingTypeGenerateable, JS_OBJECT_VARIABLE};

pub fn gen_type_checks(bindings: impl Iterator<Item = Container>) -> Tokens {
    let body = bindings.map(gen_type_check).join_with_empty_line();

    let check_function = Function::new_untyped(
        "check_bounds",
        function_args![JS_OBJECT_VARIABLE, "n_bytes", "signed", "zero_able"],
        quote! {
            if !zero_able && $JS_OBJECT_VARIABLE === 0 {
                return false
            }
            const max = BigInt(2 ** (n_bytes * BITS_PER_BYTE)), value_b = BigInt($JS_OBJECT_VARIABLE);
            if (signed) {
                const bounds = max / 2n;
                return value_b >= -bounds && value_b < bounds
            } else {
                return value_b < max && value_b >= 0
            }
        },
    );

    let check_number = Function::new_untyped(
        "check_integer_type",
        function_args![JS_OBJECT_VARIABLE, "n_bytes", "signed", "zero_able"],
        quote! {
            return (
                typeof $JS_OBJECT_VARIABLE === "number" ||
                typeof $JS_OBJECT_VARIABLE === "bigint"
            ) && Number.isInteger($JS_OBJECT_VARIABLE) && check_bounds($JS_OBJECT_VARIABLE, n_bytes, signed, zero_able)
        },
    );

    [quote!($check_function), quote!($check_number), body]
        .into_iter()
        .join_with_empty_line()
}

pub fn gen_type_check(container: Container) -> impl FormatInto<JavaScript> {
    let container_ident = ContainerIdentifierBuilder::from(&container).build();
    let body = break_long_logical_lines(container.r#type.gen_ty_check_body());

    Function::new_untyped(
        quote!(is_$container_ident),
        function_args![JS_OBJECT_VARIABLE],
        body,
    )
}
