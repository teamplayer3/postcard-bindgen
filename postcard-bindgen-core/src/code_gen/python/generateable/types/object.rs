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
        let container_ident = ContainerIdentifierBuilder::new(&self.path, self.name).build();
        quote!(serialize_$container_ident(s, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let container_ident = ContainerIdentifierBuilder::new(&self.path, self.name).build();
        quote!($(field_accessor)deserialize_$container_ident(d))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let container_ident = ContainerIdentifierBuilder::new(&self.path, self.name).build();
        quote!(assert_$container_ident($variable_path))
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        let import_path = std::iter::once("types")
            .chain(self.path.parts().skip(1))
            .collect::<Vec<_>>()
            .join(".");
        let type_alias = format!(
            "_{}",
            self.path
                .parts()
                .skip(1)
                .chain(std::iter::once(self.name))
                .collect::<Vec<_>>()
                .join("_")
        );
        import_registry.push(
            Package::Package(import_path.into()),
            ImportItem::Aliased {
                item_name: self.name.into(),
                alias: type_alias.clone().into(),
            },
        );

        quote!($type_alias)
    }
}
