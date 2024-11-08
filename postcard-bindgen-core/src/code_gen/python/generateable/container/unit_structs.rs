use genco::quote;

use crate::{
    code_gen::{
        import_registry::ImportItem,
        python::{ImportRegistry, Tokens, PYTHON_OBJECT_VARIABLE},
    },
    registry::UnitStructType,
};

use super::BindingTypeGenerateable;

impl BindingTypeGenerateable for UnitStructType {
    fn gen_ser_body(&self, _name: impl AsRef<str>) -> Tokens {
        quote!(pass)
    }

    fn gen_des_body(&self, name: impl AsRef<str>) -> Tokens {
        quote!(return $(name.as_ref())())
    }

    fn gen_ty_check_body(&self, name: impl AsRef<str>) -> Tokens {
        quote!(assert isinstance($PYTHON_OBJECT_VARIABLE, $(name.as_ref())))
    }

    fn gen_typings_body(
        &self,
        name: impl AsRef<str>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        import_registry.push(quote!(dataclasses), ImportItem::Single(quote!(dataclass)));
        quote! {
            @dataclass
            class $(name.as_ref()):
                pass
        }
    }
}
