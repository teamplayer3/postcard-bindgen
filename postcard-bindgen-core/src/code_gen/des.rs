use convert_case::{Case, Casing};
use genco::{lang::js::Tokens, quote, quote_in};

use crate::registry::{BindingType, EnumType, StructType, TupleStructType, UnitStructType};

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
    let mut tokens = Tokens::new();
    defines.as_ref().iter().for_each(|define| {
        gen_des_case(&mut tokens, define);
        tokens.append(";");
    });
    tokens
}

fn gen_des_case(tokens: &mut Tokens, define: &BindingType) {
    match define {
        BindingType::Struct(StructType { name, fields: _ })
        | BindingType::TupleStruct(TupleStructType { name, fields: _ })
        | BindingType::UnitStruct(UnitStructType { name })
        | BindingType::Enum(EnumType { name, variants: _ }) => {
            let case = format!("\"{}\"", name);
            quote_in! {*tokens =>
                case $case: return deserialize_$(name.to_case(Case::Snake).to_uppercase())(d)
            }
        }
    }
}

pub mod strukt {
    use genco::{lang::js::Tokens, quote};

    use crate::{
        registry::StructField,
        type_info::{JsType, ObjectMeta},
        utils::{StrExt, StringExt},
    };

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
        let obj_name = obj_name.as_ref();
        quote! {
            const deserialize_$(obj_name.to_obj_identifier()) = (d) => ({
                $(fields.as_ref().iter().map(gen_des_field_adapter).collect::<Vec<_>>())
            })
        }
    }

    fn gen_des_field_adapter(field: &StructField) -> Tokens {
        match &field.js_type {
            JsType::Object(m) => gen_des_function_object(&field.name, m),
            _ => gen_des_function(&field.name, &field.js_type),
        }
    }

    fn gen_des_function_object(field: impl AsRef<str>, obj_meta: &ObjectMeta) -> Tokens {
        // |<field>: deserialize_<obj_name>(d),|
        quote!($(field.as_ref()): deserialize_$(obj_meta.name.to_obj_identifier())(d),)
    }

    fn gen_des_function(field: impl AsRef<str>, ty: &JsType) -> Tokens {
        // |<field>: d.deserialize_<type>(<args...>),|
        quote!($(field.as_ref()): d.deserialize_$(ty.as_func_name())($(ty.as_js_func_args().join(","))),)
    }
}

pub mod tuple_struct {
    use convert_case::{Case, Casing};
    use genco::{prelude::JavaScript, quote, Tokens};

    use crate::type_info::{JsType, ObjectMeta};

    pub fn gen_function(
        obj_name: impl AsRef<str>,
        fields: impl AsRef<[JsType]>,
    ) -> Tokens<JavaScript> {
        let obj_name_upper = obj_name.as_ref().to_case(Case::Snake).to_uppercase();
        quote! {
            const deserialize_$obj_name_upper = (d) => ([
                $(fields.as_ref().iter().enumerate().map(|(index, field)| gen_des_field_adapter(index, field)).collect::<Vec<_>>())
            ])
        }
    }

    fn gen_des_field_adapter(index: usize, field: &JsType) -> Tokens<JavaScript> {
        match field {
            JsType::Object(m) => gen_des_function_object(index, m),
            _ => gen_des_function(index, field),
        }
    }

    fn gen_des_function_object(_index: usize, obj_meta: &ObjectMeta) -> Tokens<JavaScript> {
        // |<field>: deserialize_<obj_name>(d),|
        quote!(deserialize_$(obj_meta.name.to_case(Case::Snake).to_uppercase())(d),)
    }

    fn gen_des_function(_index: usize, ty: &JsType) -> Tokens<JavaScript> {
        // |<field>: d.deserialize_<type>(<args...>),|
        quote!(d.deserialize_$(ty.as_func_name())($(ty.as_js_func_args().join(","))),)
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
        registry::{EnumVariant, EnumVariantType, StructField},
        type_info::{JsType, ObjectMeta},
        utils::{StrExt, StringExt},
    };

    enum FieldAccess {
        Object(String),
        Array,
    }

    impl FormatInto<JavaScript> for FieldAccess {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    FieldAccess::Array => (),
                    FieldAccess::Object(n) => $n:
                })
            }
        }
    }

    pub fn gen_function(obj_name: impl AsRef<str>, variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let enumerated_variants = variants.as_ref().iter().enumerate();
        quote! {
            const deserialize_$(obj_name_upper) = (d) => {
                switch (d.deserialize_number(U32_BYTES, false)) {
                    $(enumerated_variants.to_owned().filter(|(_, v)| matches!(v.inner_type, EnumVariantType::Empty)).map(|(index, variant)| gen_des_case_unit(index, variant)).collect::<Vec<_>>())
                    $(enumerated_variants.filter(|(_, v)| !matches!(v.inner_type, EnumVariantType::Empty)).map(|(index, variant)| gen_des_case_other(index, variant)).collect::<Vec<_>>())
                    default: throw "variant not implemented"
                }
            }
        }
    }

    fn gen_des_case_unit(index: usize, variant: impl AsRef<EnumVariant>) -> Tokens {
        let field_name = &variant.as_ref().name;
        match variant.as_ref().inner_type {
            EnumVariantType::Empty => quote!(case $index: return $(quoted(field_name));),
            _ => todo!(),
        }
    }

    fn gen_des_case_other(index: usize, variant: &EnumVariant) -> Tokens {
        match &variant.inner_type {
            EnumVariantType::NewType(fields) => gen_des_var_new_type(index, &variant.name, fields),
            EnumVariantType::Tuple(fields) => gen_des_var_tuple(index, &variant.name, fields),
            _ => todo!(),
        }
    }

    fn gen_des_var_tuple(
        index: usize,
        variant_name: impl AsRef<str>,
        fields: impl AsRef<[JsType]>,
    ) -> Tokens {
        let variant_name = variant_name.as_ref();
        let body = fields
            .as_ref()
            .iter()
            .map(|field| gen_des_field_adapter(FieldAccess::Array, field))
            .collect::<Vec<_>>(); // |serialize_<obj_name>(s, v.inner);|
        quote!(case $index: return { key: $(quoted(variant_name)), inner: [$body]};)
    }

    fn gen_des_var_new_type(
        index: usize,
        variant_name: impl AsRef<str>,
        fields: impl AsRef<[StructField]>,
    ) -> Tokens {
        let variant_name = variant_name.as_ref();
        let body = fields
            .as_ref()
            .iter()
            .map(|field| {
                gen_des_field_adapter(FieldAccess::Object(field.name.to_owned()), &field.js_type)
            })
            .collect::<Vec<_>>(); // |serialize_<obj_name>(s, v.inner);|
        quote!(case $index: return { key: $(quoted(variant_name)), inner: {$body}};)
    }

    fn gen_des_field_adapter(field_access: FieldAccess, ty: &JsType) -> Tokens {
        match ty {
            JsType::Object(m) => gen_des_function_object(field_access, m),
            _ => gen_des_function(field_access, ty),
        }
    }

    fn gen_des_function_object(field_access: FieldAccess, obj_meta: &ObjectMeta) -> Tokens {
        // |<field>: deserialize_<obj_name>(d),|
        quote!($field_access deserialize_$(obj_meta.name.to_obj_identifier())(d),)
    }

    fn gen_des_function(field_access: FieldAccess, ty: &JsType) -> Tokens {
        // |<field>: d.deserialize_<type>(<args...>),|
        quote!($field_access d.deserialize_$(ty.as_func_name())($(ty.as_js_func_args().join(","))),)
    }
}
