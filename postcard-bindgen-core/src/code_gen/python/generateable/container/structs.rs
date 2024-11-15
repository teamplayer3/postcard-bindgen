use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{
            generateable::types::PythonTypeGenerateable, FieldAccessor, ImportRegistry, Tokens,
            VariableAccess, VariablePath,
        },
        utils::{ContainerFullQualifiedTypeBuilder, TokensIterExt},
    },
    registry::{ContainerInfo, StructType},
};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for StructType {
    fn gen_ser_body<'a>(&self, _container_info: ContainerInfo<'a>) -> Tokens {
        self.fields
            .iter()
            .map(|field| {
                field.v_type.gen_ser_accessor(
                    VariablePath::default().modify_push(VariableAccess::Field(field.name.into())),
                )
            })
            .join_with_line_breaks()
    }

    fn gen_des_body<'a>(&self, container_info: ContainerInfo<'a>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        let body = self
            .fields
            .iter()
            .map(|field| {
                field
                    .v_type
                    .gen_des_accessor(FieldAccessor::Object(field.name))
            })
            .join_with_comma();
        quote!(return $fully_qualified($body))
    }

    fn gen_ty_check_body<'a>(&self, container_info: ContainerInfo<'a>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        let variable_path = VariablePath::default();

        let field_checks = self
            .fields
            .iter()
            .map(|field| {
                field.v_type.gen_ty_check(
                    variable_path
                        .to_owned()
                        .modify_push(VariableAccess::Field(field.name.into())),
                )
            })
            .join_with_line_breaks();

        [
            quote!(assert isinstance($(variable_path.to_owned()), $(&fully_qualified)), "{} is not of type {}".format($variable_path, $fully_qualified)),
            field_checks
        ]
        .into_iter()
        .join_with_line_breaks()
    }

    fn gen_typings_body<'a>(
        &self,
        container_info: ContainerInfo<'a>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        let body = self
            .fields
            .iter()
            .map(|field| quote!($(field.name): $(field.v_type.gen_typings(import_registry))))
            .join_with_line_breaks();
        import_registry.push(
            Package::Extern("dataclasses".into()),
            ImportItem::Single("dataclass".into()),
        );
        quote! {
            @dataclass
            class $(container_info.name.as_str()):
                $body
        }
    }
}
