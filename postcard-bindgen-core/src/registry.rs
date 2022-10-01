use std::vec::Vec;

use crate::type_info::{GenJsBinding, JsType};

#[derive(Debug)]
pub enum BindingType {
    Struct(StructType),
    TupleStruct(TupleStructType),
    Enum(EnumType),
}

#[derive(Debug)]
// encoded into | variant index | (inner)
pub struct EnumType {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

impl EnumType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variants: Default::default(),
        }
    }

    // index is set based on order of variant registration
    pub fn register_variant(&mut self, name: String) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::Empty,
        });
    }

    pub fn register_variant_tuple(&mut self, name: String, fields: TupleFields) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::Tuple(fields.into_inner()),
        });
    }

    pub fn register_unnamed_struct(&mut self, name: String, fields: StructFields) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::UnnamedStruct(fields.into_inner()),
        })
    }
}

#[derive(Debug)]
pub struct EnumVariant {
    pub index: usize,
    pub name: String,
    pub inner_type: EnumVariantType,
}

#[derive(Debug)]
pub enum EnumVariantType {
    Empty,
    Tuple(Vec<JsType>),
    // for unnamed structs create struct with custom name ( __EnumName_Struct1)
    UnnamedStruct(Vec<StructField>),
}

#[derive(Debug)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<StructField>,
}

impl StructType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Default::default(),
        }
    }

    pub fn register_field<T: GenJsBinding>(&mut self, name: String) {
        self.fields.push(StructField {
            name,
            js_type: T::get_type(),
        })
    }
}

#[derive(Debug)]
pub struct TupleStructType {
    pub name: String,
    pub fields: Vec<JsType>,
}

impl TupleStructType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Default::default(),
        }
    }

    pub fn register_field<T: GenJsBinding>(&mut self) {
        self.fields.push(T::get_type())
    }
}

#[derive(Debug)]
pub struct StructField {
    // Tuple struct fields have no name
    pub name: String,
    pub js_type: JsType,
}

#[derive(Debug, Default)]
pub struct StructFields(Vec<StructField>);

impl StructFields {
    pub fn register_field<T: GenJsBinding>(&mut self, name: String) {
        self.0.push(StructField {
            name,
            js_type: T::get_type(),
        })
    }

    fn into_inner(self) -> Vec<StructField> {
        self.0
    }
}

#[derive(Default)]
pub struct TupleFields(Vec<JsType>);

impl TupleFields {
    pub fn register_field<T: GenJsBinding>(&mut self) {
        self.0.push(T::get_type())
    }

    fn into_inner(self) -> Vec<JsType> {
        self.0
    }
}

#[derive(Debug, Default)]
pub struct BindingsRegistry(Vec<BindingType>);

impl BindingsRegistry {
    pub fn register_struct_binding(&mut self, value: StructType) {
        self.0.push(BindingType::Struct(value));
    }

    pub fn register_tuple_struct_binding(&mut self, value: TupleStructType) {
        self.0.push(BindingType::TupleStruct(value));
    }

    pub fn register_enum_binding(&mut self, value: EnumType) {
        self.0.push(BindingType::Enum(value));
    }

    pub fn into_entries(self) -> Vec<BindingType> {
        self.0
    }
}

pub trait JsBindings {
    fn create_bindings(registry: &mut BindingsRegistry);
}

#[cfg(test)]
mod test {
    use crate::registry::{
        BindingsRegistry, EnumType, JsBindings, StructFields, StructType, TupleFields,
        TupleStructType,
    };

    #[test]
    fn test_registry_struct() {
        #[allow(unused)]
        struct Test {
            a: u8,
            b: u16,
            c: String,
        }

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = StructType::new("Test".into());

                ty.register_field::<u8>("a".into());
                ty.register_field::<u16>("b".into());
                ty.register_field::<String>("c".into());

                registry.register_struct_binding(ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
        println!("{:?}", registry)
    }

    #[test]
    fn test_registry_tuple_struct() {
        struct Test(u8, String, Vec<u8>);

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = TupleStructType::new("Test".into());

                ty.register_field::<u8>();
                ty.register_field::<String>();
                ty.register_field::<Vec<u8>>();

                registry.register_tuple_struct_binding(ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
        println!("{:?}", registry)
    }

    #[test]
    fn test_registry_enum() {
        #[allow(unused)]
        enum Test {
            A,
            B(u8),
            C { a: String, b: u16 },
        }

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = EnumType::new("Test".into());

                ty.register_variant("A".into());

                let mut fields = TupleFields::default();
                fields.register_field::<u8>();
                ty.register_variant_tuple("B".into(), fields);

                let mut fields = StructFields::default();
                fields.register_field::<String>("a".into());
                fields.register_field::<u16>("b".into());
                ty.register_unnamed_struct("C".into(), fields);

                registry.register_enum_binding(ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
        println!("{:?}", registry)
    }
}
