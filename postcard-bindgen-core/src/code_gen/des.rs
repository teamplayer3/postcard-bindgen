use genco::{
    lang::js::Tokens,
    prelude::JavaScript,
    quote, quote_in,
    tokens::{quoted, FormatInto},
};

use crate::{
    registry::{BindingType, StructField},
    type_info::{bool_to_js_bool, ArrayMeta, JsType, NumberMeta, ObjectMeta},
    utils::StringExt,
};

use super::{comma_chain, semicolon_chain};

enum FieldAccessor<'a> {
    Object(&'a str),
    Array,
}

impl<'a> FormatInto<JavaScript> for FieldAccessor<'a> {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            $(match self {
                Self::Array => (),
                Self::Object(n) => $n:
            })
        }
    }
}

fn gen_accessor(js_type: &JsType, field_accessor: FieldAccessor) -> Tokens {
    let accessor_type = js_type.as_func_name();
    match js_type {
        JsType::Array(a) => gen_accessor_array(accessor_type, a, field_accessor),
        JsType::Number(n) => gen_accessor_number(accessor_type, n, field_accessor),
        JsType::String(_) => gen_accessor_simple(accessor_type, field_accessor),
        JsType::Object(o) => gen_accessor_object(o, field_accessor),
    }
}

// quote!($(field_accessor)d.deserialize_$(ty.as_func_name())())
fn gen_accessor_simple(accessor_type: impl AsRef<str>, field_accessor: FieldAccessor) -> Tokens {
    let accessor_type = accessor_type.as_ref();
    quote!($(field_accessor)d.deserialize_$accessor_type())
}

fn gen_accessor_number(
    accessor_type: impl AsRef<str>,
    number_meta: &NumberMeta,
    field_accessor: FieldAccessor,
) -> Tokens {
    let accessor_type = accessor_type.as_ref();
    let byte_amount_str = number_meta.as_byte_js_string();
    let signed = bool_to_js_bool(number_meta.signed);
    quote!($(field_accessor)d.deserialize_$accessor_type($byte_amount_str,$signed))
}

// quote!($field_access d.deserialize_$(ty.as_func_name())(() => $(gen_des_function_nested(items_type))))
// quote!($(field.as_ref()): d.deserialize_$(ty.as_func_name())(() => $(gen_des_function_nested(items_type))))
// quote!(d.deserialize_$(ty.as_func_name())(() => $(gen_des_function_nested(items_type))))
fn gen_accessor_array(
    accessor_type: impl AsRef<str>,
    array_meta: &ArrayMeta,
    field_accessor: FieldAccessor,
) -> Tokens {
    let accessor_type = accessor_type.as_ref();
    let inner_type_accessor = gen_accessor(&array_meta.items_type, FieldAccessor::Array);
    quote!($(field_accessor)d.deserialize_$accessor_type(() => $inner_type_accessor))
}

fn gen_accessor_object(obj_meta: &ObjectMeta, field_accessor: FieldAccessor) -> Tokens {
    let obj_ident = obj_meta.name.to_obj_identifier();
    quote!($(field_accessor)deserialize_$obj_ident(d))
}

fn gen_accessor_struct(fields: impl AsRef<[StructField]>) -> Tokens {
    let body = comma_chain(
        fields
            .as_ref()
            .iter()
            .map(|field| gen_accessor(&field.js_type, FieldAccessor::Object(field.name.as_str()))),
    );
    quote!({$body})
}

fn gen_accessor_tuple(fields: impl AsRef<[JsType]>) -> Tokens {
    let body = comma_chain(
        fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(_, js_type)| gen_accessor(js_type, FieldAccessor::Array)),
    );
    quote!([$body])
}

pub fn gen_deserialize_func(defines: impl AsRef<[BindingType]>) -> Tokens {
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

fn gen_des_cases(defines: impl AsRef<[BindingType]>) -> Tokens {
    semicolon_chain(defines.as_ref().iter().map(gen_des_case))
}

fn gen_des_case(define: &BindingType) -> Tokens {
    let name = define.inner_name();
    let case_str = quoted(name.as_str());
    let type_name = name.to_obj_identifier();
    quote!(case $case_str: return deserialize_$type_name(d))
}

pub mod strukt {
    use genco::{lang::js::Tokens, quote};

    use crate::{registry::StructField, utils::StrExt};

    use super::gen_accessor_struct;

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
        let obj_name = obj_name.as_ref().to_obj_identifier();
        let body = gen_accessor_struct(fields);
        quote!(const deserialize_$obj_name = (d) => ($body))
    }
}

pub mod tuple_struct {
    use genco::{lang::js::Tokens, quote};

    use crate::{type_info::JsType, utils::StrExt};

    use super::gen_accessor_tuple;

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[JsType]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let body = gen_accessor_tuple(fields);
        quote!(const deserialize_$obj_name_upper = (d) => ($body))
    }
}

pub mod enum_ty {
    use genco::{lang::js::Tokens, quote, tokens::quoted};

    use crate::{
        code_gen::semicolon_chain,
        registry::{EnumVariant, EnumVariantType},
        utils::StrExt,
    };

    use super::{gen_accessor_struct, gen_accessor_tuple};

    pub fn gen_function(obj_name: impl AsRef<str>, variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let enumerated_variants = variants.as_ref().iter().enumerate();
        quote! {
            const deserialize_$(obj_name_upper) = (d) => {
                switch (d.deserialize_number(U32_BYTES, false)) {
                    $(semicolon_chain(enumerated_variants.to_owned().filter(|(_, v)| matches!(v.inner_type, EnumVariantType::Empty)).map(|(index, variant)| gen_case_for_variant(index, variant))))
                    $(semicolon_chain(enumerated_variants.filter(|(_, v)| !matches!(v.inner_type, EnumVariantType::Empty)).map(|(index, variant)| gen_case_for_variant(index, variant))))
                    default: throw "variant not implemented"
                }
            }
        }
    }

    fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Tokens {
        let variant_name = quoted(&variant.name);
        let body = match &variant.inner_type {
            EnumVariantType::Empty => Tokens::new(),
            EnumVariantType::NewType(fields) => quote!(, inner: $(gen_accessor_struct(fields))),
            EnumVariantType::Tuple(fields) => quote!(, inner: $(gen_accessor_tuple(fields))),
        };
        quote!(case $index: return { key: $variant_name $body})
    }
}
