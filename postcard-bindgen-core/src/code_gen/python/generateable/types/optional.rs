use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{FieldAccessor, ImportRegistry, Tokens, VariablePath, PYTHON_OBJECT_VARIABLE},
    },
    type_info::OptionalMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for OptionalMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let type_accessor = self.inner.gen_ser_accessor(variable_path.to_owned());
        quote! {
            if $variable_path is not None:
                s.serialize_number(U32_BYTES, False, 1)
                $type_accessor
            else:
                s.serialize_number(U32_BYTES, False, 0)
        }
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_accessor = self.inner.gen_des_accessor(FieldAccessor::None);
        quote! {
            $(field_accessor) None if d.deserialize_number(U32_BYTES, False) == 0 else $inner_accessor
        }
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let assert_func_name = quote!(assert_$(variable_path.to_owned().into_string("_")));
        let assert_item_type_check_func = quote! {
            def $(&assert_func_name)($PYTHON_OBJECT_VARIABLE):
                $(self.inner.gen_ty_check(VariablePath::default()))
        };

        quote! {
            $assert_item_type_check_func
            if $(variable_path.to_owned()) is not None:
                $assert_func_name($variable_path)
        }
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        import_registry.push(
            Package::Extern("typing".into()),
            ImportItem::Single("Optional".into()),
        );
        quote!(Optional[$(self.inner.gen_typings(import_registry))])
    }
}
