use convert_case::{Case, Casing};
use genco::{prelude::js::Tokens, quote, quote_in};
use postcard_bindgen_core::{StrExt, StringExt};
use regex_macro::regex;
use serde_derive_internals::ast::Field;
use syn::TypePath;

enum Direction {
    Serialize,
    Deserialize,
}

pub fn gen_ser_der_funcs(obj_name: impl AsRef<str>, fields: &[Field]) -> Tokens {
    let obj_name = obj_name.as_ref();

    let mut ser_body = Tokens::new();
    quote_in! {ser_body =>
        $(fields
            .iter()
            .map(|field| gen_field_func(field, Direction::Serialize))
            .collect::<Vec<_>>())
    };

    let mut des_body = Tokens::new();
    quote_in! {des_body =>
        return {
            $(fields.iter().map(|field|
                quote! {
                    $(gen_field_func(field, Direction::Deserialize))

                }
            ).collect::<Vec<_>>())
        }
    }

    quote!(
        $(gen_const_function(
            format!("serialize_{}", obj_name.to_case(Case::Snake).to_uppercase()),
            Some(vec!["s", "v"]),
            ser_body,
        ))
        $(gen_const_function(
            format!("deserialize_{}", obj_name.to_case(Case::Snake).to_uppercase()),
            Some(vec!["d"]),
            des_body
        ))
    )
}

pub fn gen_const_function(
    func_name: impl AsRef<str>,
    params: Option<Vec<impl AsRef<str>>>,
    body: Tokens,
) -> Tokens {
    let func_name = func_name.as_ref();
    let params = params
        .map(|v| {
            v.iter()
                .map(|param| param.as_ref())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    quote!(
        const $func_name = ($params) => {
            $body
        }
    )
}

fn gen_field_func(field: &Field, direction: Direction) -> Tokens {
    let (func_accessor, left_hand, func_divider) = match direction {
        Direction::Serialize => ("s", Tokens::new(), ";"),
        Direction::Deserialize => (
            "d",
            quote!($(field.original.ident.as_ref().unwrap().to_string()):),
            ",",
        ),
    };
    quote!($left_hand $(func_accessor).$(gen_func_name_and_head(field.original.ident.as_ref().unwrap().to_string(), field.ty, direction))$func_divider)
}

fn gen_func_name_and_head(ident: impl AsRef<str>, ty: &syn::Type, direction: Direction) -> Tokens {
    use syn::Type::*;
    match ty {
        Slice(_) => unimplemented!(),
        Array(_) => unimplemented!(),
        Ptr(_) => unimplemented!(),
        Reference(_) => unimplemented!(),
        BareFn(_) => unimplemented!(),
        Never(_) => unimplemented!(),
        Tuple(_) => unimplemented!(),
        Path(inner) => match_path_type_to_serialize_func_ending(ident, inner, direction),
        TraitObject(_) => unimplemented!(),
        ImplTrait(_) => unimplemented!(),
        Paren(_) => unimplemented!(),
        Group(_) => unimplemented!(),
        Infer(_) => unimplemented!(),
        Macro(_) => unimplemented!(),
        Verbatim(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

const U8_BYTES_CONST: &str = "U8_BYTES";
const U16_BYTES_CONST: &str = "U16_BYTES";
const U32_BYTES_CONST: &str = "U32_BYTES";
const U64_BYTES_CONST: &str = "U64_BYTES";
const U128_BYTES_CONST: &str = "U128_BYTES";
const USIZE_BYTES_CONST: &str = "USIZE_BYTES";

fn number_type_to_const_js_type(c: &str) -> &'static str {
    match c {
        "8" => U8_BYTES_CONST,
        "16" => U16_BYTES_CONST,
        "32" => U32_BYTES_CONST,
        "64" => U64_BYTES_CONST,
        "128" => U128_BYTES_CONST,
        "SIZE" => USIZE_BYTES_CONST,
        _ => unreachable!(),
    }
}

fn match_path_type_to_serialize_func_ending(
    ident: impl AsRef<str>,
    path: &TypePath,
    direction: Direction,
) -> Tokens {
    let number_matcher = regex!(r"^([u|i])(\d+)$");
    let string_matcher = regex!(r"(?:alloc|std)::string::String");
    let array_matcher = regex!(r"(?:alloc|std)::vec::Vec<([u|i]\d+)>");

    let path_string = quote::quote!(#path).to_string().trim_all();
    let path_str = path_string.as_str();
    let mut tokens = Tokens::new();

    if let Some(captures) = number_matcher.captures(path_str) {
        gen_accessor_number(
            &mut tokens,
            number_type_to_const_js_type(captures.get(2).unwrap().as_str()),
            captures.get(1).unwrap().as_str().is_signed_pref().unwrap(),
            ident,
            direction,
        );
    } else if string_matcher.is_match(path_str) {
        gen_accessor_string(&mut tokens, ident, direction)
    } else if let Some(captures) = array_matcher.captures(path_str) {
        if let Some(captures) = number_matcher.captures(captures.get(1).unwrap().as_str()) {
            gen_accessor_array(
                &mut tokens,
                number_type_to_const_js_type(captures.get(2).unwrap().as_str()),
                captures.get(1).unwrap().as_str().is_signed_pref().unwrap(),
                ident,
                direction,
            )
        }
    }

    tokens
}

fn gen_accessor_array(
    tokens: &mut Tokens,
    n_bytes: &'static str,
    signed: bool,
    ident: impl AsRef<str>,
    direction: Direction,
) {
    let signed_bool = if signed { "true" } else { "false" };
    match direction {
        Direction::Serialize => quote_in! {*tokens =>
            serialize_array($(n_bytes), $(signed_bool), v.$(ident.as_ref()))
        },
        Direction::Deserialize => quote_in! {*tokens =>
            deserialize_array($(n_bytes), $(signed_bool))
        },
    }
}

fn gen_accessor_string(tokens: &mut Tokens, ident: impl AsRef<str>, direction: Direction) {
    match direction {
        Direction::Serialize => quote_in! {*tokens =>
            serialize_string(v.$(ident.as_ref()))
        },
        Direction::Deserialize => quote_in! {*tokens =>
            deserialize_string()
        },
    }
}

fn gen_accessor_number(
    tokens: &mut Tokens,
    n_bytes: &'static str,
    signed: bool,
    ident: impl AsRef<str>,
    direction: Direction,
) {
    let signed_bool = if signed { "true" } else { "false" };
    match direction {
        Direction::Serialize => quote_in! {*tokens =>
            serialize_number($(n_bytes), $(signed_bool), v.$(ident.as_ref()))
        },
        Direction::Deserialize => quote_in! {*tokens =>
            deserialize_number($(n_bytes), $(signed_bool))
        },
    }
}

#[cfg(test)]
mod test {
    use genco::quote;

    use super::gen_const_function;

    #[test]
    fn test_gen_function() {
        let body = quote!(return x.test());
        let func = gen_const_function("my_test", Some(vec!["x"]), body);
        println!("{:?}", func.to_string())
    }
}
