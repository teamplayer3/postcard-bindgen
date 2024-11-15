use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{ImportRegistry, Tokens, PYTHON_OBJECT_VARIABLE},
        utils::ContainerFullQualifiedTypeBuilder,
    },
    registry::{ContainerInfo, UnitStructType},
};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for UnitStructType {
    fn gen_ser_body(&self, _container_info: ContainerInfo<'_>) -> Tokens {
        quote!(pass)
    }

    fn gen_des_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        quote!(return $fully_qualified())
    }

    fn gen_ty_check_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        quote!(assert isinstance($PYTHON_OBJECT_VARIABLE, $fully_qualified))
    }

    fn gen_typings_body(
        &self,
        container_info: ContainerInfo<'_>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        import_registry.push(
            Package::Extern("dataclasses".into()),
            ImportItem::Single("dataclass".into()),
        );
        quote! {
            @dataclass
            class $(container_info.name):
                pass
        }
    }
}
