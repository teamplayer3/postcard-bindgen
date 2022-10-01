use convert_case::{Case, Casing};
use genco::{lang::js::Tokens, quote, quote_in};

use crate::{
    registry::{BindingType, StructField, StructType},
    type_info::{JsType, ObjectMeta},
};

pub fn gen_serialize_func(defines: &Vec<BindingType>) -> Tokens {
    quote!(
        module.exports.serialize = (type, value) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const s = new Serializer()
            switch (type) {
                $(gen_ser_cases(defines))
            }
            return s.finish()
        }
    )
}

fn gen_ser_cases(defines: impl AsRef<[BindingType]>) -> Tokens {
    let mut tokens = Tokens::new();
    defines.as_ref().iter().for_each(|define| {
        gen_ser_case(&mut tokens, define);
        tokens.append(";");
    });
    tokens
}

fn gen_ser_case(tokens: &mut Tokens, define: &BindingType) {
    match define {
        BindingType::Struct(StructType { name, fields: _ }) => {
            let case = format!("\"{}\"", name);
            let type_name = name.to_case(Case::Snake).to_uppercase();
            quote_in! {*tokens =>
                case $case: if (is_$(type_name.as_str())(value)) { serialize_$(type_name)(s, value) } else throw "value has wrong format"; break
            }
        }
        _ => todo!(),
    }
}

pub fn gen_ser_obj_function(
    obj_name: impl AsRef<str>,
    fields: impl AsRef<[StructField]>,
) -> Tokens {
    let obj_name_upper = obj_name.as_ref().to_case(Case::Snake).to_uppercase();
    quote! {
        const serialize_$(obj_name_upper) = (s, v) => {
            $(fields.as_ref().iter().map(gen_ser_field_adapter).collect::<Vec<_>>())
        }
    }
}

fn gen_ser_field_adapter(field: &StructField) -> Tokens {
    match &field.js_type {
        JsType::Object(m) => gen_ser_function_object(&field.name, m),
        _ => gen_ser_function(&field.name, &field.js_type),
    }
}

fn gen_ser_function_object(field: impl AsRef<str>, obj_meta: &ObjectMeta) -> Tokens {
    // |serialize_<obj_name>(s, v.<field>);|
    quote!(serialize_$(obj_meta.name.to_case(Case::Snake).to_uppercase())(s, v.$(field.as_ref()));)
}

fn gen_ser_function(field: impl AsRef<str>, ty: &JsType) -> Tokens {
    // |s.serialize_<type>(<args...>, v.<field>);|
    quote!(s.serialize_$(ty.as_func_name())($(ty.as_js_func_args().join(",")),v.$(field.as_ref()));)
}
