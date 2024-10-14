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
    fn gen_ser_body(&self) -> Tokens {
        quote!(pass)
    }

    fn gen_des_body(&self) -> Tokens {
        quote!(return $(self.name)())
    }

    fn gen_ty_check_body(&self) -> Tokens {
        quote!(assert isinstance($PYTHON_OBJECT_VARIABLE, $(self.name)))
    }

    fn gen_typings_body(&self, import_registry: &mut ImportRegistry) -> Tokens {
        import_registry.push(quote!(dataclasses), ImportItem::Single(quote!(dataclass)));
        quote! {
            @dataclass
            class $(self.name):
                pass
        }
    }
}
