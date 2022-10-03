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
    gen_deserialize_func, gen_ser_des_classes, gen_ser_des_functions, gen_serialize_func,
    type_checking::gen_type_checkings,
};
use registry::BindingType;

pub enum ArchPointerLen {
    U32,
    U64,
}

impl ArchPointerLen {
    #[allow(unused)]
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
    let ser_des_body = gen_ser_des_functions(&tys);
    let type_checks = gen_type_checkings(&tys);
    quote!(
        $(gen_ser_des_classes())
        $ser_des_body
        $type_checks
        $(gen_serialize_func(&tys))
        $(gen_deserialize_func(&tys))
    )
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
