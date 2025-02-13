use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{generateable::types::bool::bool_to_python_bool, ImportRegistry, Tokens},
        utils::TokensIterExt,
    },
    type_info::NumberMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for NumberMeta {
    fn gen_ser_accessor(&self, variable_path: crate::code_gen::python::VariablePath) -> Tokens {
        let byte_amount_str = self.as_byte_string();
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!(s.serialize_number_float($byte_amount_str, $variable_path))
            }
            NumberMeta::Integer { signed, .. } => {
                let signed = bool_to_python_bool(*signed);
                quote!(s.serialize_number($byte_amount_str, $signed, $variable_path))
            }
        }
    }

    fn gen_des_accessor(&self, field_accessor: crate::code_gen::python::FieldAccessor) -> Tokens {
        let byte_amount_str = self.as_byte_string();
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!($(field_accessor)d.deserialize_number_float($byte_amount_str))
            }
            NumberMeta::Integer { signed, .. } => {
                let signed = bool_to_python_bool(*signed);
                quote!($(field_accessor)d.deserialize_number($byte_amount_str, $signed))
            }
        }
    }

    fn gen_ty_check(&self, variable_path: crate::code_gen::python::VariablePath) -> Tokens {
        let byte_amount_str = self.as_byte_string();
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!(assert isinstance($(variable_path.to_owned()), float), "{} is not a float".format($variable_path))
            }
            NumberMeta::Integer {
                signed, zero_able, ..
            } => {
                let signed = bool_to_python_bool(*signed);
                let zero_able = bool_to_python_bool(*zero_able);
                [
                    quote!(assert isinstance($(variable_path.to_owned()), int), "{} is not an int".format($(variable_path.to_owned()))),
                    quote!(check_bounds($byte_amount_str, $signed, $(variable_path.to_owned()), $zero_able))
                ]
                .into_iter()
                .join_with_line_breaks()
            }
        }
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        match self {
            NumberMeta::FloatingPoint { .. } => {
                quote!(float)
            }
            NumberMeta::Integer {
                bytes,
                signed,
                zero_able,
            } => {
                let int_type = rust_int_to_python_type(*bytes, *signed, !*zero_able);
                import_registry.push(Package::Root, ImportItem::Single("basic_types".into()));
                quote!(basic_types.$int_type)
            }
        }
    }
}

fn rust_int_to_python_type(bytes: usize, signed: bool, non_zero: bool) -> String {
    let bits = match bytes {
        1 => "8",
        2 => "16",
        4 => "32",
        8 => "64",
        16 => "128",
        _ => unimplemented!(),
    };
    let sign = match signed {
        true => "i",
        false => "u",
    };

    if non_zero {
        format!("NonZero{}{bits}", sign.to_uppercase())
    } else {
        format!("{sign}{bits}")
    }
}
