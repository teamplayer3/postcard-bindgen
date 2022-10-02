mod code_gen;
pub mod registry;
pub mod type_info;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use genco::{prelude::JavaScript, quote, Tokens};

use code_gen::{
    des::{self, gen_des_obj_function, gen_deserialize_func},
    gen_ser_des_classes,
    ser::{gen_ser_obj_function, gen_serialize_func, tuple_struct},
    type_checking::{self, gen_check_func},
};
use registry::{BindingType, StructType, TupleStructType};

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
        BindingType::Enum(_ty) => todo!(),
        BindingType::Struct(ty) => generate_js_object(ty),
        BindingType::TupleStruct(ty) => generate_js_object_tuple(ty),
    });
    let type_check_body = tys.iter().map(|ty| match ty {
        BindingType::Enum(_ty) => todo!(),
        BindingType::Struct(ty) => gen_check_func(&ty.name, &ty.fields),
        BindingType::TupleStruct(ty) => {
            type_checking::tuple_struct::gen_check_func(&ty.name, &ty.fields)
        }
    });
    quote!(
        $(gen_ser_des_classes(ArchPointerLen::U32))
        $(ser_des_body.map(|body| body.to_string().unwrap()).collect::<Vec<_>>().join("\n"))
        $(type_check_body.map(|body| body.to_string().unwrap()).collect::<Vec<_>>().join("\n"))
        $(gen_serialize_func(&tys))
        $(gen_deserialize_func(&tys))
    )
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

pub trait StringExt {
    fn trim_all(self) -> Self;
    fn is_signed_pref(&self) -> Option<bool>;
}

pub trait StrExt {
    fn is_signed_pref(&self) -> Option<bool>;
}

impl StringExt for String {
    fn trim_all(mut self) -> Self {
        self.retain(|c| !c.is_whitespace());
        self
    }

    fn is_signed_pref(&self) -> Option<bool> {
        is_signed_pref(self.as_str())
    }
}

impl<'a> StrExt for &'a str {
    fn is_signed_pref(&self) -> Option<bool> {
        is_signed_pref(self)
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
