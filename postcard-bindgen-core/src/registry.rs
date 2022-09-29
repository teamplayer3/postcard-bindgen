use std::vec::Vec;

use crate::type_info::{GenJsBinding, JsType};

// encoded into | variant index | (inner)
pub struct EnumType {
    name: String,
    variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    index: usize,
    name: String,
    // for unnamed structs create struct with custom name ( __EnumName_Struct1)
    inner_type: Option<JsType>,
}

pub struct StructType {
    name: String,
    fields: Vec<StructField>,
}

pub struct TupleStructType {
    name: String,
    fields: Vec<StructField>,
}

pub struct StructField {
    // Tuple struct fields have no name
    name: Option<String>,
    js_type: JsType,
}

pub struct BindingsRegistry {
    struct_type: Option<StructType>,
}

impl BindingsRegistry {
    pub fn register_struct_binding(&mut self, name: String) {
        self.struct_type = Some(StructType {
            name,
            fields: Default::default(),
        })
    }

    pub fn register_struct_field<T: GenJsBinding>(&mut self, name: String) {
        self.struct_type.as_mut().unwrap().fields.push(StructField {
            name: Some(name),
            js_type: T::get_type(),
        })
    }
}

pub trait JsBindings {
    fn create_bindings(&mut self) {}

    fn register_binding() {}
}
