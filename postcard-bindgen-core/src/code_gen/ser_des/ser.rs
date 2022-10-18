use genco::{lang::js::Tokens, quote, tokens::quoted};

use crate::{
    code_gen::{generateable::js_types::*, utils::semicolon_chain},
    registry::{BindingType, StructField},
    type_info::JsType,
    utils::StrExt,
};

fn gen_accessors_tuple(fields: impl AsRef<[JsType]>, field_access: ser::InnerTypeAccess) -> Tokens {
    semicolon_chain(fields.as_ref().iter().enumerate().map(|(index, field)| {
        field.gen_ser_accessor(field_access, ser::FieldAccessor::Array(index))
    }))
}

fn gen_accessors_struct(
    fields: impl AsRef<[StructField]>,
    field_access: ser::InnerTypeAccess,
) -> Tokens {
    semicolon_chain(fields.as_ref().iter().map(|field| {
        field
            .js_type
            .gen_ser_accessor(field_access, ser::FieldAccessor::Object(field.name))
    }))
}

pub fn gen_serialize_func(defines: impl AsRef<[BindingType]>) -> Tokens {
    let switch_body = gen_ser_cases(defines);
    quote!(
        module.exports.serialize = (type, value) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const s = new Serializer()
            switch (type) {
                $switch_body
            }
            return s.finish()
        }
    )
}

fn gen_ser_cases(defines: impl AsRef<[BindingType]>) -> Tokens {
    semicolon_chain(defines.as_ref().iter().map(gen_ser_case))
}

fn gen_ser_case(define: &BindingType) -> Tokens {
    let name = define.inner_name();
    let case_str = quoted(name);
    let type_name = name.to_obj_identifier();
    quote!(case $case_str: if (is_$(type_name.as_str())(value)) { serialize_$(type_name)(s, value) } else throw "value has wrong format"; break)
}

pub mod strukt {
    use genco::{lang::js::Tokens, quote};

    use crate::{code_gen::generateable::js_types::*, registry::StructField, utils::StrExt};

    use super::gen_accessors_struct;

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let body = gen_accessors_struct(fields, ser::InnerTypeAccess::Direct);
        quote! {
            const serialize_$(obj_name_upper) = (s, v) => { $body }
        }
    }
}

pub mod tuple_struct {
    use genco::{lang::js::Tokens, quote};

    use crate::{code_gen::generateable::js_types::*, type_info::JsType, utils::StrExt};

    use super::gen_accessors_tuple;

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[JsType]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let body = gen_accessors_tuple(fields, ser::InnerTypeAccess::Direct);
        quote! {
            const serialize_$(obj_name_upper) = (s, v) => { $body }
        }
    }
}

pub mod enum_ty {
    use genco::{
        lang::js::Tokens,
        prelude::JavaScript,
        quote, quote_in,
        tokens::{quoted, FormatInto},
    };

    use crate::{
        code_gen::{generateable::js_types::*, utils::semicolon_chain, JS_ENUM_VARIANT_KEY},
        registry::{EnumVariant, EnumVariantType},
        utils::StrExt,
    };

    use super::{gen_accessors_struct, gen_accessors_tuple};

    pub fn gen_function(obj_name: impl AsRef<str>, variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let enumerated_variants = variants.as_ref().iter().enumerate();
        let switch_body = semicolon_chain(
            enumerated_variants.map(|(index, variant)| gen_case_for_variant(index, variant)),
        );
        quote! {
            const serialize_$(obj_name_upper) = (s, v) => {
                switch (v.$JS_ENUM_VARIANT_KEY) {
                    $switch_body
                }
            }
        }
    }

    enum CaseBody {
        Body(Tokens),
        None,
    }

    impl FormatInto<JavaScript> for CaseBody {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    CaseBody::Body(b) => $b;,
                    CaseBody::None => ()
                })
            }
        }
    }

    fn gen_case_for_variant(index: usize, variant: &EnumVariant) -> Tokens {
        let variant_name = quoted(variant.name);
        let body = match &variant.inner_type {
            EnumVariantType::Empty => CaseBody::None,
            EnumVariantType::Tuple(fields) => {
                CaseBody::Body(gen_accessors_tuple(fields, ser::InnerTypeAccess::EnumInner))
            }
            EnumVariantType::NewType(fields) => CaseBody::Body(gen_accessors_struct(
                fields,
                ser::InnerTypeAccess::EnumInner,
            )),
        };

        quote!(case $variant_name: s.serialize_number(U32_BYTES, false, $index); $body break)
    }
}
