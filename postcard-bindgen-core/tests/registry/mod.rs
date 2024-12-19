use std::collections::HashMap;

use postcard_bindgen_core::{
    path::Path,
    registry::{
        BindingsRegistry, EnumType, StructFields, StructType, TupleFields, TupleStructType,
        UnitStructType,
    },
    type_info::{GenJsBinding, ObjectMeta, ValueType},
};

macro_rules! dummy_struct {
    ($path:ident,$name:ident) => {
        with_builtin_macros::with_builtin! {
            let $sname = concat_idents!(Dummy, $name) in {
                struct $sname;
                impl GenJsBinding for $sname {
                    fn get_type() -> ValueType {
                        ValueType::Object(ObjectMeta {
                            name: stringify!($name),
                            path: Path::new("main_crate", "::"),
                        })
                    }
                }
            }

        }
    };
}

pub fn init_registry() -> BindingsRegistry {
    let mut registry = BindingsRegistry::default();

    dummy_struct!(main_crate, StructType);

    let mut struct_type = StructType::new();
    struct_type.register_field::<u32>("field_1");
    struct_type.register_field::<String>("field_2");
    struct_type.register_field::<Vec<u32>>("field_3");
    struct_type.register_field::<Vec<DummyStructType>>("field_4");
    struct_type.register_field::<core::ops::Range<u32>>("field_5");
    struct_type.register_field::<HashMap<String, u32>>("field_6");
    struct_type.register_field::<HashMap<u32, u32>>("field_7");
    struct_type.register_field::<Option<u32>>("field_8");
    struct_type.register_field::<bool>("field_9");
    struct_type.register_field::<(u32, String)>("field_10");
    struct_type.register_field::<[String; 3]>("field_11");
    struct_type.register_field::<&[u32]>("field_12");
    struct_type.register_field::<f32>("field_13");

    registry.register_struct_binding("StructType", "main_crate", struct_type);

    let struct_type = UnitStructType::new();
    registry.register_unit_struct_binding("UnitStructType", "main_crate", struct_type);

    let mut struct_type = TupleStructType::new();
    struct_type.register_field::<u32>();
    struct_type.register_field::<String>();
    registry.register_tuple_struct_binding("TupleStructType", "main_crate", struct_type);

    let mut enum_type = EnumType::new();
    enum_type.register_variant("AVariant");

    let mut fields = TupleFields::default();
    fields.register_field::<u32>();
    fields.register_field::<String>();
    enum_type.register_variant_tuple("BVariant", fields);

    let mut fields = StructFields::default();
    fields.register_field::<u32>("field_1");
    fields.register_field::<String>("field_2");

    fields.register_field::<DummyStructType>("struct_type");
    enum_type.register_unnamed_struct("CVariant", fields);

    registry.register_enum_binding("EnumType", "main_crate", enum_type);

    registry
}
