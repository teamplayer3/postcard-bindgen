use convert_case::{Case, Casing};
use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
        utils::ContainerIdentifierBuilder,
    },
    type_info::ObjectMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for ObjectMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let container_ident =
            ContainerIdentifierBuilder::new(self.path.clone().into_buf(), self.name).build();
        quote!(serialize_$container_ident(s, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let container_ident =
            ContainerIdentifierBuilder::new(self.path.clone().into_buf(), self.name).build();
        quote!($(field_accessor)deserialize_$container_ident(d))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let container_ident =
            ContainerIdentifierBuilder::new(self.path.clone().into_buf(), self.name).build();
        quote!(assert_$container_ident($variable_path))
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        let mut container_path = self.path.clone().into_buf();
        // remove the main crates name from the path
        container_path.pop_front();

        let mut import_path = container_path.clone();
        import_path.push(format!("_{}", self.name.to_case(Case::Snake)));

        import_path.push_front("types");

        container_path.push(self.name);
        let type_alias = format!("_{}", String::from(container_path.into_path("_")));

        import_registry.push(
            Package::Intern(import_path),
            ImportItem::Aliased {
                item_name: self.name.into(),
                alias: type_alias.clone().into(),
            },
        );

        quote!($type_alias)
    }
}
