use genco::quote;

use crate::{
    code_gen::{
        import_registry::ImportItem,
        python::{
            generateable::types::PythonTypeGenerateable, FieldAccessor, ImportRegistry, Tokens,
            VariableAccess, VariablePath,
        },
        utils::TokensIterExt,
    },
    registry::StructType,
};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for StructType {
    fn gen_ser_body(&self, _name: impl AsRef<str>) -> Tokens {
        self.fields
            .iter()
            .map(|field| {
                field.v_type.gen_ser_accessor(
                    VariablePath::default().modify_push(VariableAccess::Field(field.name.into())),
                )
            })
            .join_with_line_breaks()
    }

    fn gen_des_body(&self, name: impl AsRef<str>) -> Tokens {
        let body = self
            .fields
            .iter()
            .map(|field| {
                field
                    .v_type
                    .gen_des_accessor(FieldAccessor::Object(field.name))
            })
            .join_with_comma();
        quote!(return $(name.as_ref())($body))
    }

    fn gen_ty_check_body(&self, name: impl AsRef<str>) -> Tokens {
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
            quote!(assert isinstance($(variable_path.to_owned()), $(name.as_ref())), "{} is not of type {}".format($variable_path, $(name.as_ref()))),
            field_checks
        ]
        .into_iter()
        .join_with_line_breaks()
    }

    fn gen_typings_body(
        &self,
        name: impl AsRef<str>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        let body = self
            .fields
            .iter()
            .map(|field| quote!($(field.name): $(field.v_type.gen_typings(import_registry))))
            .join_with_line_breaks();
        import_registry.push(quote!(dataclasses), ImportItem::Single(quote!(dataclass)));
        quote! {
            @dataclass
            class $(name.as_ref()):
                $body
        }
    }
}
