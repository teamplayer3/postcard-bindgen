use alloc::vec::Vec;

use crate::type_info::{GenJsBinding, ValueType};

#[derive(Debug)]
pub enum BindingType {
    Struct(StructType),
    TupleStruct(TupleStructType),
    UnitStruct(UnitStructType),
    Enum(EnumType),
}

impl BindingType {
    pub fn inner_name(&self) -> &'static str {
        match self {
            Self::Struct(StructType { name, .. }) => name,
            Self::TupleStruct(TupleStructType { name, .. }) => name,
            Self::Enum(EnumType { name, .. }) => name,
            Self::UnitStruct(UnitStructType { name }) => name,
        }
    }

    pub fn inner_path(&self) -> &'static str {
        match self {
            Self::Struct(StructType { path, .. }) => path,
            Self::TupleStruct(_) => "",
            Self::Enum(_) => "",
            Self::UnitStruct(_) => "",
        }
    }
}

#[derive(Debug)]
// encoded into | variant index | (inner)
pub struct EnumType {
    pub name: &'static str,
    pub variants: Vec<EnumVariant>,
}

impl EnumType {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            variants: Default::default(),
        }
    }

    // index is set based on order of variant registration
    pub fn register_variant(&mut self, name: &'static str) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::Empty,
        });
    }

    pub fn register_variant_tuple(&mut self, name: &'static str, fields: TupleFields) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::Tuple(fields.into_inner()),
        });
    }

    pub fn register_unnamed_struct(&mut self, name: &'static str, fields: StructFields) {
        self.variants.push(EnumVariant {
            index: self.variants.len(),
            name,
            inner_type: EnumVariantType::NewType(fields.into_inner()),
        })
    }
}

#[derive(Debug)]
pub struct EnumVariant {
    pub index: usize,
    pub name: &'static str,
    pub inner_type: EnumVariantType,
}

impl AsRef<EnumVariant> for EnumVariant {
    fn as_ref(&self) -> &EnumVariant {
        self
    }
}

#[derive(Debug)]
pub enum EnumVariantType {
    Empty,
    Tuple(Vec<ValueType>),
    // for unnamed structs create struct with custom name ( __EnumName_Struct1)
    NewType(Vec<StructField>),
}

#[derive(Debug)]
pub struct StructType {
    pub name: &'static str,
    pub path: &'static str,
    pub fields: Vec<StructField>,
}

impl StructType {
    pub fn new(name: &'static str, path: &'static str) -> Self {
        Self {
            name,
            path,
            fields: Default::default(),
        }
    }

    pub fn register_field<T: GenJsBinding>(&mut self, name: &'static str) {
        self.fields.push(StructField {
            name,
            v_type: T::get_type(),
        })
    }
}

#[derive(Debug)]
pub struct TupleStructType {
    pub name: &'static str,
    pub fields: Vec<ValueType>,
}

impl TupleStructType {
    pub fn new(name: &'static str) -> Self {
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
pub struct UnitStructType {
    pub name: &'static str,
}

impl UnitStructType {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct StructField {
    pub name: &'static str,
    pub v_type: ValueType,
}

#[derive(Debug, Default)]
pub struct StructFields(Vec<StructField>);

impl StructFields {
    pub fn register_field<T: GenJsBinding>(&mut self, name: &'static str) {
        self.0.push(StructField {
            name,
            v_type: T::get_type(),
        })
    }

    fn into_inner(self) -> Vec<StructField> {
        self.0
    }
}

#[derive(Default)]
pub struct TupleFields(Vec<ValueType>);

impl TupleFields {
    pub fn register_field<T: GenJsBinding>(&mut self) {
        self.0.push(T::get_type())
    }

    fn into_inner(self) -> Vec<ValueType> {
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

    pub fn register_unit_struct_binding(&mut self, value: UnitStructType) {
        self.0.push(BindingType::UnitStruct(value));
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
            c: &'static str,
        }

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = StructType::new("Test".into(), "");

                ty.register_field::<u8>("a".into());
                ty.register_field::<u16>("b".into());
                ty.register_field::<&str>("c".into());

                registry.register_struct_binding(ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
    }

    #[test]
    fn test_registry_tuple_struct() {
        #[allow(dead_code)]
        struct Test(u8, &'static str, &'static [u8]);

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = TupleStructType::new("Test".into());

                ty.register_field::<u8>();
                ty.register_field::<&str>();
                ty.register_field::<&[u8]>();

                registry.register_tuple_struct_binding(ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
    }

    #[test]
    fn test_registry_enum() {
        #[allow(unused)]
        enum Test {
            A,
            B(u8),
            C { a: &'static str, b: u16 },
        }

        impl JsBindings for Test {
            fn create_bindings(registry: &mut BindingsRegistry) {
                let mut ty = EnumType::new("Test".into());

                ty.register_variant("A".into());

                let mut fields = TupleFields::default();
                fields.register_field::<u8>();
                ty.register_variant_tuple("B".into(), fields);

                let mut fields = StructFields::default();
                fields.register_field::<&str>("a".into());
                fields.register_field::<u16>("b".into());
                ty.register_unnamed_struct("C".into(), fields);

                registry.register_enum_binding(ty);
            }
        }

        let mut registry = BindingsRegistry::default();
        Test::create_bindings(&mut registry);
    }
}
