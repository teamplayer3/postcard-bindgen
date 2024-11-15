use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::{
        js::{FieldAccessor, VariablePath},
        utils::{ContainerFullQualifiedTypeBuilder, ContainerIdentifierBuilder},
    },
    type_info::ObjectMeta,
};

use super::JsTypeGenerateable;

impl JsTypeGenerateable for ObjectMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let container_ident = ContainerIdentifierBuilder::from(self).build();
        quote!(serialize_$container_ident(s, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let container_ident = ContainerIdentifierBuilder::from(self).build();
        quote!($(field_accessor)deserialize_$container_ident(d))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let container_ident = ContainerIdentifierBuilder::from(self).build();
        quote!(is_$container_ident($variable_path))
    }

    fn gen_ts_type(&self) -> Tokens {
        let full_qualified = ContainerFullQualifiedTypeBuilder::from(self).build();
        quote!($full_qualified)
    }
}
