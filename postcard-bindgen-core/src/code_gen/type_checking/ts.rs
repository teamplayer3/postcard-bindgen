use genco::{
    prelude::{js::Tokens, JavaScript},
    quote, quote_in,
    tokens::{quoted, FormatInto},
};

use crate::{
    code_gen::{
        utils::{colon_chain, comma_chain, divider_chain, line_brake_chain},
        JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE,
    },
    registry::{BindingType, EnumVariant, EnumVariantType, StructField},
    type_info::{ArrayMeta, JsType, ObjectMeta},
};

pub fn gen_ts_typings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    quote!(
        $(gen_number_decls())

        $(gen_bindings_types(&bindings))

        $(gen_type_decl(&bindings))
        $(gen_value_type_decl(bindings))

        $(gen_ser_des_decls())
    )
}

fn gen_number_decls() -> Tokens {
    quote!(
        declare type u8 = number
        declare type u16 = number
        declare type u32 = number
        declare type u64 = number
        declare type u128 = number
        declare type usize = number
        declare type i8 = number
        declare type i16 = number
        declare type i32 = number
        declare type i64 = number
        declare type i128 = number
        declare type isize = number
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

fn js_type_format_into(tokens: &mut Tokens, ty: &JsType) {
    quote_in! { *tokens =>
        $(match ty {
            JsType::Number(m) => $(m.as_ts_type()),
            JsType::Array(ArrayMeta {items_type}) => $(items_type.as_ref())[],
            JsType::Object(ObjectMeta {name}) => $(*name),
            JsType::Optional(inner) => $(inner.as_ref()) | undefined,
            JsType::String(_) => string
        })
    }
}

impl FormatInto<JavaScript> for &JsType {
    fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
        js_type_format_into(tokens, self)
    }
}

impl FormatInto<JavaScript> for JsType {
    fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
        js_type_format_into(tokens, &self)
    }
}

fn gen_bindings_types(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(|b| gen_binding_type(b)))
}

fn gen_binding_type(binding: &BindingType) -> Tokens {
    let name = binding.inner_name();
    let body = gen_binding_type_structure(binding);
    quote!(export type $name = $body)
}

fn gen_binding_type_structure(binding: &BindingType) -> Tokens {
    match binding {
        BindingType::TupleStruct(t) => gen_tuple_typings(&t.fields),
        BindingType::Enum(e) => gen_enum_typings(&e.variants),
        BindingType::Struct(s) => gen_struct_typings(&s.fields),
        BindingType::UnitStruct(_) => gen_unit_struct_typings(),
    }
}

fn gen_tuple_typings(fields: impl AsRef<[JsType]>) -> Tokens {
    let body = comma_chain(fields.as_ref().iter().map(|f| quote!($f)));
    quote!([$body])
}

fn gen_struct_typings(fields: impl AsRef<[StructField]>) -> Tokens {
    let body = comma_chain(
        fields
            .as_ref()
            .iter()
            .map(|f| quote!($(f.name): $(&f.js_type))),
    );
    quote!({ $body })
}

fn gen_unit_struct_typings() -> Tokens {
    quote!({})
}

fn gen_enum_typings(variants: impl AsRef<[EnumVariant]>) -> Tokens {
    let body = divider_chain(variants.as_ref().iter().map(gen_variant_typings));
    quote!($body)
}

fn gen_variant_typings(variant: &EnumVariant) -> Tokens {
    let name = quoted(variant.name);
    match &variant.inner_type {
        EnumVariantType::Empty => quote!({ $JS_ENUM_VARIANT_KEY: $name }),
        t => {
            let body = match t {
                EnumVariantType::Tuple(t) => gen_tuple_typings(t),
                EnumVariantType::NewType(n) => gen_struct_typings(n),
                _ => unreachable!(),
            };
            quote!({ $JS_ENUM_VARIANT_KEY: $name, $JS_ENUM_VARIANT_VALUE: $body })
        }
    }
}
