use convert_case::{Case, Casing};
use genco::{lang::js::Tokens, prelude::JavaScript, quote, quote_in, tokens::FormatInto};

use crate::registry::{BindingType, EnumType, StructType, TupleStructType, UnitStructType};

enum FieldAccess {
    Object(String),
    Array(usize),
}

impl FormatInto<JavaScript> for FieldAccess {
    fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
        quote_in! { *tokens =>
            $(match self {
                FieldAccess::Array(i) => [$i],
                FieldAccess::Object(n) => .$n
            })
        }
    }
}

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
        BindingType::Struct(StructType { name, fields: _ })
        | BindingType::TupleStruct(TupleStructType { name, fields: _ })
        | BindingType::UnitStruct(UnitStructType { name })
        | BindingType::Enum(EnumType { name, variants: _ }) => {
            let case = format!("\"{}\"", name);
            let type_name = name.to_case(Case::Snake).to_uppercase();
            quote_in! {*tokens =>
                case $case: if (is_$(type_name.as_str())(value)) { serialize_$(type_name)(s, value) } else throw "value has wrong format"; break
            }
        }
    }
}

pub mod strukt {
    use convert_case::{Case, Casing};
    use genco::{lang::js::Tokens, quote};

    use crate::{
        registry::StructField,
        type_info::{ArrayMeta, JsType, ObjectMeta},
        utils::StringExt,
    };

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
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
        quote!(serialize_$(obj_meta.name.to_obj_identifier())(s, v.$(field.as_ref()));)
    }

    fn gen_ser_function(field: impl AsRef<str>, ty: &JsType) -> Tokens {
        // |s.serialize_<type>(<args...>, v.<field>);|
        match ty {
            JsType::Array(ArrayMeta { items_type }) => {
                quote!(s.serialize_$(ty.as_func_name())((s, v) => $(gen_ser_function_nested(&*items_type)),v.$(field.as_ref())))
            }
            _ => {
                quote!(s.serialize_$(ty.as_func_name())($(ty.as_js_func_args().join(",")),v.$(field.as_ref()));)
            }
        }
    }

    fn gen_ser_function_nested(ty: &JsType) -> Tokens {
        match ty {
            JsType::Array(ArrayMeta { items_type }) => {
                quote!(s.serialize_$(ty.as_func_name())((s, v) => $(gen_ser_function_nested(&*items_type)),v))
            }
            _ => {
                quote!(s.serialize_$(ty.as_func_name())($(ty.as_js_func_args().join(",")),v))
            }
        }
    }
}

pub mod tuple_struct {
    use convert_case::{Case, Casing};
    use genco::{lang::js::Tokens, quote};

    use crate::type_info::{ArrayMeta, JsType, ObjectMeta};

    pub fn gen_function(obj_name: impl AsRef<str>, fields: impl AsRef<[JsType]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_case(Case::Snake).to_uppercase();
        quote! {
            const serialize_$(obj_name_upper) = (s, v) => {
                $(fields.as_ref().iter().enumerate().map(|(index,field)| gen_ser_field_adapter(index, field)).collect::<Vec<_>>())
            }
        }
    }

    fn gen_ser_field_adapter(index: usize, field: &JsType) -> Tokens {
        match field {
            JsType::Object(m) => gen_ser_function_object(index, m),
            _ => gen_ser_function(index, field),
        }
    }

    fn gen_ser_function_object(index: usize, obj_meta: &ObjectMeta) -> Tokens {
        // |serialize_<obj_name>(s, v.<field>);|
        quote!(serialize_$(obj_meta.name.to_case(Case::Snake).to_uppercase())(s, v[$index]);)
    }

    fn gen_ser_function(index: usize, ty: &JsType) -> Tokens {
        // |s.serialize_<type>(<args...>, v.<field>);|
        match ty {
            JsType::Array(ArrayMeta { items_type }) => {
                quote!(s.serialize_$(ty.as_func_name())((s, v) => $(gen_ser_function_nested(&*items_type)),v[$index]))
            }
            _ => {
                quote!(s.serialize_$(ty.as_func_name())($(ty.as_js_func_args().join(",")),v[$index]);)
            }
        }
    }

    fn gen_ser_function_nested(ty: &JsType) -> Tokens {
        match ty {
            JsType::Array(ArrayMeta { items_type }) => {
                quote!(s.serialize_$(ty.as_func_name())((s, v) => $(gen_ser_function_nested(&*items_type)),v))
            }
            _ => {
                quote!(s.serialize_$(ty.as_func_name())($(ty.as_js_func_args().join(",")),v))
            }
        }
    }
}

pub mod enum_ty {
    use genco::{lang::js::Tokens, quote, tokens::quoted};

    use crate::{
        registry::{EnumVariant, EnumVariantType, StructField},
        type_info::{JsType, ObjectMeta},
        utils::{StrExt, StringExt},
    };

    use super::FieldAccess;

    pub fn gen_function(obj_name: impl AsRef<str>, variants: impl AsRef<[EnumVariant]>) -> Tokens {
        let obj_name_upper = obj_name.as_ref().to_obj_identifier();
        let enumerated_variants = variants.as_ref().iter().enumerate();
        quote! {
            const serialize_$(obj_name_upper) = (s, v) => {
                if (typeof v === "string") {
                    switch (v) {
                        $(enumerated_variants.to_owned().filter(|(_, v)| matches!(v.inner_type, EnumVariantType::Empty)).map(|(index, variant)| gen_ser_case_unit(index, variant)).collect::<Vec<_>>())
                    }
                } else {
                    switch (v.key) {
                        $(enumerated_variants.filter(|(_, v)| !matches!(v.inner_type, EnumVariantType::Empty)).map(|(index, variant)| gen_ser_case_other(index, variant)).collect::<Vec<_>>())
                    }
                }

            }
        }
    }

    fn gen_ser_case_unit(index: usize, variant: impl AsRef<EnumVariant>) -> Tokens {
        let variant_name = &variant.as_ref().name;
        quote!(
            case $(quoted(variant_name)): s.serialize_number(U32_BYTES, false, $index); break;
        )
    }

    fn gen_ser_case_other(index: usize, variant: &EnumVariant) -> Tokens {
        match &variant.inner_type {
            EnumVariantType::NewType(fields) => gen_ser_var_new_type(index, &variant.name, fields),
            EnumVariantType::Tuple(fields) => gen_ser_var_tuple_type(index, &variant.name, fields),
            _ => todo!(),
        }
    }

    fn gen_ser_var_tuple_type(
        index: usize,
        variant_name: impl AsRef<str>,
        fields: impl AsRef<[JsType]>,
    ) -> Tokens {
        let variant_name = variant_name.as_ref();
        let body = fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(field_index, field)| {
                gen_ser_field_adapter(FieldAccess::Array(field_index), field)
            })
            .collect::<Vec<_>>(); // |serialize_<obj_name>(s, v.inner);|
        quote!(case $(quoted(variant_name)): s.serialize_number(U32_BYTES, false, $index); $body break;)
    }

    fn gen_ser_var_new_type(
        index: usize,
        variant_name: impl AsRef<str>,
        fields: impl AsRef<[StructField]>,
    ) -> Tokens {
        let variant_name = variant_name.as_ref();
        let body = fields
            .as_ref()
            .iter()
            .map(|field| {
                gen_ser_field_adapter(FieldAccess::Object(field.name.to_owned()), &field.js_type)
            })
            .collect::<Vec<_>>(); // |serialize_<obj_name>(s, v.inner);|
        quote!(case $(quoted(variant_name)): s.serialize_number(U32_BYTES, false, $index); $body break;)
    }

    fn gen_ser_field_adapter(field_access: FieldAccess, ty: &JsType) -> Tokens {
        match ty {
            JsType::Object(m) => gen_ser_function_object(field_access, m),
            ty => gen_ser_function(field_access, ty),
        }
    }

    fn gen_ser_function_object(field_access: FieldAccess, obj_meta: &ObjectMeta) -> Tokens {
        // |serialize_<obj_name>(s, v.<field>);|
        quote!(serialize_$(obj_meta.name.to_obj_identifier())(s, v.inner$field_access);)
    }

    fn gen_ser_function(field_access: FieldAccess, ty: &JsType) -> Tokens {
        // |s.serialize_<type>(<args...>, v.<field>);|
        quote!(s.serialize_$(ty.as_func_name())($(ty.as_js_func_args().join(",")),v.inner$field_access);)
    }
}
