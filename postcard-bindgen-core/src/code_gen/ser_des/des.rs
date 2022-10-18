use genco::{lang::js::Tokens, quote, tokens::quoted};

use crate::{
    code_gen::{
        generateable::js_types::*,
        utils::{comma_chain, semicolon_chain},
    },
    registry::{BindingType, StructField},
    type_info::JsType,
    utils::StrExt,
};

fn gen_accessor_struct(fields: impl AsRef<[StructField]>) -> Tokens {
    let body = comma_chain(fields.as_ref().iter().map(|field| {
        field
            .js_type
            .gen_des_accessor(des::FieldAccessor::Object(field.name))
    }));
    quote!({ $body })
}

fn gen_accessor_tuple(fields: impl AsRef<[JsType]>) -> Tokens {
    let body = comma_chain(
        fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(_, js_type)| js_type.gen_des_accessor(des::FieldAccessor::Array)),
    );
    quote!([ $body ])
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
    let case_str = quoted(name);
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
        code_gen::{utils::semicolon_chain, JS_ENUM_VARIANT_KEY, JS_ENUM_VARIANT_VALUE},
        registry::{EnumVariant, EnumVariantType},
        utils::StrExt,
    };

    use super::{gen_accessor_struct, gen_accessor_tuple};

    pub fn gen_function(obj_name: impl AsRef<str>, variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let enumerated_variants = variants.as_ref().iter().enumerate();
        let switch_body = semicolon_chain(
            enumerated_variants.map(|(index, variant)| gen_case_for_variant(index, variant)),
        );
        quote! {
            const deserialize_$(obj_name_upper) = (d) => {
                switch (d.deserialize_number(U32_BYTES, false)) {
                    $switch_body
                    default: throw "variant not implemented"
                }
            }
        }
    }

    fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Tokens {
        let variant_name = quoted(variant.name);
        let body = match &variant.inner_type {
            EnumVariantType::Empty => Tokens::new(),
            EnumVariantType::NewType(fields) => {
                quote!(, $JS_ENUM_VARIANT_VALUE: $(gen_accessor_struct(fields)))
            }
            EnumVariantType::Tuple(fields) => {
                quote!(, $JS_ENUM_VARIANT_VALUE: $(gen_accessor_tuple(fields)))
            }
        };
        quote!(case $index: return { $JS_ENUM_VARIANT_KEY: $variant_name $body})
    }
}
