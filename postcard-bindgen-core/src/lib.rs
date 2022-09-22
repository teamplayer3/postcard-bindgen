mod code_gen;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use genco::quote;

use code_gen::{des::gen_deserialize_func, gen_ser_des_classes, ser::gen_serialize_func};

pub trait JsExportable {
    const JS_STRING: &'static str;
    const TYPE_IDENT: &'static str;

    fn js_bindings() -> JsTyping {
        JsTyping {
            js_bindings: Self::JS_STRING,
            type_ident: Self::TYPE_IDENT,
        }
    }
}

pub struct JsTyping {
    js_bindings: &'static str,
    type_ident: &'static str,
}

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

pub fn export_js_bindings(
    path: &Path,
    defines: Vec<JsTyping>,
    pointer_type: ArchPointerLen,
) -> io::Result<()> {
    let js_string = quote!(
        $(gen_ser_des_classes(pointer_type))
        $(defines.iter().map(|define| define.js_bindings).collect::<Vec<_>>().join("\n"))
        $(gen_serialize_func(&defines))
        $(gen_deserialize_func(&defines))
    )
    .to_file_string()
    .unwrap();

    let mut file = File::create(path)?;
    file.write_all(js_string.as_str().as_bytes())?;
    Ok(())
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
