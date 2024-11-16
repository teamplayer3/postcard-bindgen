use genco::{quote, tokens::quoted};

use crate::{
    code_gen::{
        python::{
            generateable::types::PythonTypeGenerateable, FieldAccessor, ImportRegistry, Tokens,
            VariableAccess, VariablePath, PYTHON_OBJECT_VARIABLE,
        },
        utils::{ContainerFullQualifiedTypeBuilder, TokensIterExt},
    },
    registry::{ContainerInfo, TupleStructType},
};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for TupleStructType {
    fn gen_ser_body(&self, _container_info: ContainerInfo<'_>) -> Tokens {
        self.fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                field.gen_ser_accessor(
                    VariablePath::default().modify_push(VariableAccess::Indexed(index)),
                )
            })
            .join_with_line_breaks()
    }

    fn gen_des_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        let body = self
            .fields
            .iter()
            .map(|v_type| v_type.gen_des_accessor(FieldAccessor::None))
            .join_with_comma();
        // <struct_name>(#0, #1, ...)
        quote!(return $fully_qualified($body))
    }

    fn gen_ty_check_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        let type_checks = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.gen_ty_check(
                    VariablePath::new(PYTHON_OBJECT_VARIABLE.to_owned())
                        .to_owned()
                        .modify_push(VariableAccess::Indexed(i)),
                )
            })
            .join_with_line_breaks();
        [
            quote!(assert isinstance($PYTHON_OBJECT_VARIABLE, tuple), "{} is not a tuple".format($(&fully_qualified))),
            quote!(assert len($PYTHON_OBJECT_VARIABLE) == $(self.fields.len()), "{} is not of length {}".format($fully_qualified, $(self.fields.len()))),
            type_checks
        ]
        .into_iter()
        .join_with_line_breaks()
    }

    fn gen_typings_body(
        &self,
        container_info: ContainerInfo<'_>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        let types = self
            .fields
            .iter()
            .map(|f| f.gen_typings(import_registry))
            .collect::<Vec<_>>();

        let types_comma_chained = types.iter().cloned().join_with_comma();
        let constructor_args = types
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, t)| quote!(_$i: $t))
            .join_with_comma();
        let pass_on_args = types
            .into_iter()
            .enumerate()
            .map(|(i, _)| quote!(_$i))
            .join_with_comma_min_one();

        let class_name = container_info.name.as_ref();

        quote! {
            class $class_name(tuple[$types_comma_chained]):

                def __new__(cls, $(&constructor_args)):
                    return super($class_name, cls).__new__(cls, ($pass_on_args))

                def __init__(self, $constructor_args):
                    pass

                def __str__(self) -> str:
                    return "{}{}".format($(quoted(class_name)), super().__str__())

                def __format__(self, format_spec: str) -> str:
                    return super().__format__(format_spec)

                def __repr__(self) -> str:
                    return super().__repr__()
        }
    }
}
