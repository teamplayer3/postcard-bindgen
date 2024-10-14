use genco::{prelude::python::Tokens, quote, quote_in};

use crate::{
    code_gen::{
        python::{generateable::container::BindingTypeGenerateable, PYTHON_OBJECT_VARIABLE},
        utils::{TokensBranchedIterExt, TokensIterExt},
    },
    registry::BindingType,
    utils::StrExt,
};

pub fn gen_serializer_code() -> Tokens {
    quote! {
        import struct

        from .util import *

        class Serializer:
            def __init__(self):
                self.bytes = []

            def finish(self) -> bytes:
                return bytes(self.bytes)

            def push_n(self, bytes_in):
                self.bytes.extend(bytes_in)

            def serialize_bool(self, value):
                self.serialize_number(U8_BYTES, False, 1 if value else 0)

            def serialize_number(self, n_bytes, signed, value):
                if n_bytes == U8_BYTES:
                    self.bytes.append(value)
                elif n_bytes in {U16_BYTES, U32_BYTES, U64_BYTES, U128_BYTES}:
                    value_b = int(value)
                    buffer = varint(n_bytes, zig_zag(n_bytes, value_b) if signed else value_b)
                    self.push_n(buffer)
                else:
                    raise Exception("byte count not supported")

            def serialize_number_float(self, n_bytes, value):
                if n_bytes == U32_BYTES:
                    b_buffer = struct.pack("<f", value)
                elif n_bytes == U64_BYTES:
                    b_buffer = struct.pack("<d", value)
                else:
                    raise Exception("byte count not supported")
                self.push_n(b_buffer)

            def serialize_string(self, s):
                self.push_n(varint(U32_BYTES, len(s)))
                self.push_n([ord(c) for c in s])

            def serialize_array(self, ser, array, length):
                if length is None:
                    self.push_n(varint(U32_BYTES, len(array)))
                [ser(self, array[i]) for i in range(len(array) if length is None else length)]

            def serialize_map(self, ser, map_obj):
                entries = list(map_obj.items())
                self.push_n(varint(U32_BYTES, len(entries)))
                for k, v in entries:
                    ser(self, k, v)
    }
}

pub fn gen_ser_functions(bindings: impl AsRef<[BindingType]>) -> Tokens {
    bindings
        .as_ref()
        .iter()
        .map(gen_ser_function_for_type)
        .join_with_empty_line()
}

fn gen_ser_function_for_type(binding_type: &BindingType) -> Tokens {
    let obj_name = binding_type.inner_name().to_obj_identifier();
    let ser_body = binding_type.gen_ser_body();
    quote! {
        def serialize_$(&obj_name)(s, $PYTHON_OBJECT_VARIABLE):
            $ser_body
    }
}

pub fn gen_serialize_func(tys: impl AsRef<[BindingType]>, runtime_type_checks: bool) -> Tokens {
    let all_bindings = tys
        .as_ref()
        .iter()
        .map(|d| quote!($(d.inner_name())))
        .collect::<Vec<_>>();

    let type_check = if all_bindings.len() == 1 {
        quote!($(all_bindings.first().unwrap()))
    } else {
        quote!(Union[$(all_bindings.into_iter().join_with_comma())])
    };

    let ser_switch = tys
        .as_ref()
        .iter()
        .map(|t| gen_ser_case(t, runtime_type_checks))
        .map(|(condition, body)| (Some(condition), body))
        .chain([(
            None,
            quote!(raise TypeError("{} not serializable".format(type(value)))),
        )])
        .join_if_branched();

    let mut tokens = Tokens::new();
    if runtime_type_checks {
        quote_in!(tokens=> from .type_checks import *);
        tokens.push();
    }

    quote_in! {tokens=>
        def serialize(value: $type_check) -> bytes:
            s = Serializer()

            $ser_switch

            return s.finish()
    }

    tokens
}

fn gen_ser_case(define: &BindingType, runtime_type_checks: bool) -> (Tokens, Tokens) {
    let name = define.inner_name();
    let type_name = name.to_obj_identifier();

    let case_body = {
        let mut tokens = Tokens::new();

        if runtime_type_checks {
            quote_in!(tokens=> assert_$(&type_name)(value));
            tokens.push();
        }
        quote_in!(tokens=> serialize_$(&type_name)(s, value));

        tokens
    };

    (quote!(isinstance(value, $(&type_name))), case_body)
}
