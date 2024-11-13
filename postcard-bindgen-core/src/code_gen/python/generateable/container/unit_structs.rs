use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{ImportRegistry, Tokens, PYTHON_OBJECT_VARIABLE},
        utils::ContainerFullQualifiedTypeBuilder,
    },
    registry::UnitStructType,
    utils::ContainerPath,
};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for UnitStructType {
    fn gen_ser_body<'a>(
        &self,
        _name: impl AsRef<str>,
        _path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens {
        quote!(pass)
    }

    fn gen_des_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens {
        let fully_qualified =
            ContainerFullQualifiedTypeBuilder::new(path.as_ref(), name.as_ref()).build();
        quote!(return $fully_qualified())
    }

    fn gen_ty_check_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens {
        let fully_qualified =
            ContainerFullQualifiedTypeBuilder::new(path.as_ref(), name.as_ref()).build();
        quote!(assert isinstance($PYTHON_OBJECT_VARIABLE, $fully_qualified))
    }

    fn gen_typings_body<'a>(
        &self,
        name: impl AsRef<str>,
        _path: impl AsRef<ContainerPath<'a>>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        import_registry.push(
            Package::Extern("dataclasses".into()),
            ImportItem::Single("dataclass".into()),
        );
        quote! {
            @dataclass
            class $(name.as_ref()):
                pass
        }
    }
}
