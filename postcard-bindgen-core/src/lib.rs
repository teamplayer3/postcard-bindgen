mod code_gen;
pub mod registry;
pub mod type_info;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use convert_case::{Case, Casing};
use genco::{prelude::JavaScript, quote, Tokens};

use code_gen::{
    des::{self, gen_des_obj_function, gen_deserialize_func},
    gen_ser_des_classes,
    ser::{self, gen_ser_obj_function, gen_serialize_func, tuple_struct},
    type_checking::gen_type_checkings,
};
use registry::{BindingType, EnumType, StructType, TupleStructType};

pub enum ArchPointerLen {
    U32,
    U64,
}

impl ArchPointerLen {
    pub(crate) fn into_bytes(self) -> usize {
        match self {
            ArchPointerLen::U32 => 4,
            ArchPointerLen::U64 => 8,
        }
    }
}

#[macro_export]
macro_rules! generate_bindings {
    ($( $x:ty ),*) => {
        {
            let mut reg = postcard_bindgen::BindingsRegistry::default();
            $(
                <$x as postcard_bindgen::JsBindings>::create_bindings(&mut reg);
            )*
            postcard_bindgen::generate_js(reg.into_entries()).to_file_string().unwrap()
        }
    };
}

pub fn export_bindings(path: &Path, bindings: impl AsRef<str>) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(bindings.as_ref().as_bytes())?;
    Ok(())
}

pub fn generate_js(tys: Vec<BindingType>) -> Tokens<JavaScript> {
    let ser_des_body = tys.iter().map(|ty| match ty {
        BindingType::Enum(ty) => generate_js_enum(ty),
        BindingType::Struct(ty) => generate_js_object(ty),
        BindingType::TupleStruct(ty) => generate_js_object_tuple(ty),
        BindingType::UnitStruct(ty) => generate_js_obj_unit(&ty.name),
    });
    let type_checks = gen_type_checkings(&tys);
    quote!(
        $(gen_ser_des_classes(ArchPointerLen::U32))
        $(ser_des_body.map(|body| body.to_string().unwrap()).collect::<Vec<_>>().join("\n"))
        $type_checks
        $(gen_serialize_func(&tys))
        $(gen_deserialize_func(&tys))
    )
}

fn generate_js_obj_unit(name: impl AsRef<str>) -> Tokens<JavaScript> {
    quote! {
        $(gen_ser_obj_function(name.as_ref(), &[]))
        $(gen_des_obj_function(name, &[]))
    }
}

fn generate_js_object(ty: &StructType) -> Tokens<JavaScript> {
    let obj_name = &ty.name;
    quote! {
        $(gen_ser_obj_function(obj_name, &ty.fields))
        $(gen_des_obj_function(obj_name, &ty.fields))
    }
}

fn generate_js_object_tuple(ty: &TupleStructType) -> Tokens<JavaScript> {
    let obj_name = &ty.name;
    quote! {
        $(tuple_struct::gen_ser_tuple_obj_function(obj_name, &ty.fields))
        $(des::tuple_struct::gen_des_obj_function(obj_name, &ty.fields))
    }
}

fn generate_js_enum(ty: &EnumType) -> Tokens<JavaScript> {
    let obj_name = &ty.name;
    quote! {
        $(ser::enum_ty::gen_ser_enum_function(obj_name, &ty.variants))
        $(des::enum_ty::gen_des_enum_function(obj_name, &ty.variants))
    }
}

pub trait StringExt {
    fn trim_all(self) -> Self;
    fn is_signed_pref(&self) -> Option<bool>;
    fn to_obj_identifier(&self) -> Self;
}

pub trait StrExt {
    fn is_signed_pref(&self) -> Option<bool>;
    fn to_obj_identifier(&self) -> String;
}

impl StringExt for String {
    fn trim_all(mut self) -> Self {
        self.retain(|c| !c.is_whitespace());
        self
    }

    fn is_signed_pref(&self) -> Option<bool> {
        is_signed_pref(self.as_str())
    }

    fn to_obj_identifier(&self) -> Self {
        self.to_case(Case::Snake).to_uppercase()
    }
}

impl<'a> StrExt for &'a str {
    fn is_signed_pref(&self) -> Option<bool> {
        is_signed_pref(self)
    }

    fn to_obj_identifier(&self) -> String {
        self.to_case(Case::Snake).to_uppercase()
    }
}

fn is_signed_pref(s: &str) -> Option<bool> {
    if s.len() != 1 {
        return None;
    }
    match s.chars().next().unwrap() {
        'i' => Some(true),
        'u' => Some(false),
        _ => None,
    }
}
