use genco::{quote, tokens::quoted};

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{
            generateable::types::PythonTypeGenerateable, FieldAccessor, ImportRegistry, Tokens,
            PYTHON_OBJECT_VARIABLE,
        },
        utils::{ContainerFullQualifiedTypeBuilder, TokensBranchedIterExt, TokensIterExt},
        variable_path::{VariableAccess, VariablePath},
    },
    registry::{EnumType, EnumVariant, EnumVariantType},
};

use super::{BindingTypeGenerateable, ContainerInfo};

impl BindingTypeGenerateable for EnumType {
    fn gen_ser_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        self.variants
            .iter()
            .map(|v| {
                let variant_name = quote!($(&fully_qualified)_$(v.name));

                let ser_fields = [quote!(s.serialize_number(U32_BYTES, False, $(v.index)))]
                    .into_iter()
                    .chain([match &v.inner_type {
                        EnumVariantType::Empty => quote!(),
                        EnumVariantType::NewType(fields) => fields
                            .iter()
                            .map(|f| {
                                f.v_type.gen_ser_accessor(
                                    VariablePath::default()
                                        .modify_push(VariableAccess::Field(f.name.to_owned())),
                                )
                            })
                            .join_with_line_breaks(),
                        EnumVariantType::Tuple(fields) => fields
                            .iter()
                            .enumerate()
                            .map(|(i, f)| {
                                f.gen_ser_accessor(
                                    VariablePath::default().modify_push(VariableAccess::Indexed(i)),
                                )
                            })
                            .join_with_line_breaks(),
                    }])
                    .join_with_line_breaks();

                (
                    Some(quote!(isinstance($PYTHON_OBJECT_VARIABLE, $variant_name))),
                    ser_fields,
                )
            })
            .chain([(
                None,
                quote!(raise TypeError("variant {} not exists".format($PYTHON_OBJECT_VARIABLE))),
            )])
            .join_if_branched()
    }

    fn gen_des_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        let switch = self
            .variants
            .iter()
            .map(|v| {
                let variant_name = quote!($(&fully_qualified)_$(v.name));

                let constructor_args = match &v.inner_type {
                    EnumVariantType::Empty => quote!(),
                    EnumVariantType::NewType(fields) => fields.iter().map(
                        |f| quote!($(f.name) = $(f.v_type.gen_des_accessor(FieldAccessor::None))),
                    ).join_with_comma(),
                    EnumVariantType::Tuple(fields) => fields.iter().map(
                        |f| quote!($(f.gen_des_accessor(FieldAccessor::None))),
                    ).join_with_comma(),
                };
                (
                    Some(quote!(variant_index == $(v.index))),
                    quote!(return $variant_name($constructor_args)),
                )
            })
            .chain([(
                None,
                quote!(raise TypeError("variant index {} not exists".format(variant_index))),
            )])
            .join_if_branched();

        quote! {
            variant_index = d.deserialize_number(U32_BYTES, False)
            $switch
        }
    }

    fn gen_ty_check_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container_info).build();
        let assert_funcs = self
            .variants
            .iter()
            .map(|v| {
                let body = match &v.inner_type {
                    EnumVariantType::Empty => quote!(pass),
                    EnumVariantType::NewType(fields) => fields
                        .iter()
                        .map(|f| {
                            f.v_type.gen_ty_check(
                                VariablePath::default()
                                    .modify_push(VariableAccess::Field(f.name.to_owned())),
                            )
                        })
                        .join_with_line_breaks(),
                    EnumVariantType::Tuple(fields) => fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            f.gen_ty_check(
                                VariablePath::default().modify_push(VariableAccess::Indexed(i)),
                            )
                        })
                        .join_with_line_breaks(),
                };
                quote! {
                    def assert_$(v.name)($PYTHON_OBJECT_VARIABLE):
                        $body

                }
            })
            .join_with_line_breaks();

        let switch = self
            .variants
            .iter()
            .map(|v| {
                let variant_name = quote!($(&fully_qualified)_$(v.name));
                (
                    Some(quote!(isinstance($PYTHON_OBJECT_VARIABLE, $variant_name))),
                    quote!(assert_$(v.name)($PYTHON_OBJECT_VARIABLE)),
                )
            })
            .chain([(
                None,
                quote!(raise TypeError("variant {} not exists".format($PYTHON_OBJECT_VARIABLE))),
            )])
            .join_if_branched();

        quote! {
            $assert_funcs

            $switch
        }
    }

    fn gen_typings_body(
        &self,
        container_info: ContainerInfo<'_>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        let variants = self
            .variants
            .iter()
            .map(|v| gen_variant_typings(container_info.name.as_ref(), v, import_registry))
            .join_with_empty_line();

        quote! {
            class $(container_info.name):
                pass

            $variants
        }
    }
}

fn gen_variant_typings(
    enum_name: impl AsRef<str>,
    variant: impl AsRef<EnumVariant>,
    import_registry: &mut ImportRegistry,
) -> Tokens {
    let enum_name = enum_name.as_ref();
    let variant = variant.as_ref();

    let variant_name = quote!($(enum_name)_$(variant.name));

    match &variant.inner_type {
        EnumVariantType::Empty => quote! {
            class $variant_name($enum_name):
                pass
        },
        EnumVariantType::NewType(fields) => {
            let fields = fields
                .iter()
                .map(|f| quote!($(f.name): $(f.v_type.gen_typings(import_registry))))
                .join_with_line_breaks();

            import_registry.push(
                Package::Extern("dataclasses".into()),
                ImportItem::Single("dataclass".into()),
            );
            quote! {
                @dataclass
                class $variant_name($enum_name):
                    $fields
            }
        }
        EnumVariantType::Tuple(fields) => {
            let types = fields
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

            quote! {
                class $(&variant_name)($enum_name, tuple[$types_comma_chained]):

                    def __new__(cls, $(&constructor_args)):
                        return super($(&variant_name), cls).__new__(cls, ($pass_on_args))

                    def __init__(self, $constructor_args):
                        pass

                    def __str__(self) -> str:
                        return "{}{}".format($(quoted(variant_name)), super().__str__())

                    def __format__(self, format_spec: str) -> str:
                        return super().__format__(format_spec)

                    def __repr__(self) -> str:
                        return super().__repr__()
            }
        }
    }
}
