use convert_case::{Case, Casing};
use genco::{lang::js::Tokens, quote, quote_in};

use crate::{
    registry::{BindingType, StructField, StructType, TupleStructType},
    type_info::{JsType, ObjectMeta},
};

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
        | BindingType::TupleStruct(TupleStructType { name, fields: _ }) => {
            let case = format!("\"{}\"", name);
            quote_in! {*tokens =>
                case $case: return deserialize_$(name.to_case(Case::Snake).to_uppercase())(d)
            }
        }
        _ => todo!(),
    }
}

pub fn gen_des_obj_function(
    obj_name: impl AsRef<str>,
    fields: impl AsRef<[StructField]>,
) -> Tokens {
    let obj_name_upper = obj_name.as_ref().to_case(Case::Snake).to_uppercase();
    quote! {
        const deserialize_$obj_name_upper = (d) => ({
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
    quote!($(field.as_ref()): deserialize_$(obj_meta.name.to_case(Case::Snake).to_uppercase())(d),)
}

fn gen_des_function(field: impl AsRef<str>, ty: &JsType) -> Tokens {
    // |<field>: d.deserialize_<type>(<args...>),|
    quote!($(field.as_ref()): d.deserialize_$(ty.as_func_name())($(ty.as_js_func_args().join(","))),)
}

pub mod tuple_struct {
    use convert_case::{Case, Casing};
    use genco::{prelude::JavaScript, quote, Tokens};

    use crate::type_info::{JsType, ObjectMeta};

    pub fn gen_des_obj_function(
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
