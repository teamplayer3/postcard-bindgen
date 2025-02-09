use genco::{lang::Python, prelude::python::Tokens, quote, quote_in, tokens::FormatInto};

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{
            generateable::container::BindingTypeGenerateable, Function, ImportRegistry,
            PYTHON_OBJECT_VARIABLE,
        },
        utils::{
            ContainerFullQualifiedTypeBuilder, ContainerIdentifierBuilder, TokensBranchedIterExt,
            TokensIterExt,
        },
    },
    function_args,
    registry::Container,
};

pub fn gen_serializer_code() -> Tokens {
    quote! {
        import struct

        from .util import *

        class Serializer:
            def __init__(self):
                self.bytes = bytearray()

            def finish(self) -> bytes:
                return bytes(self.bytes)

            def push_n(self, bytes_in):
                self.bytes.extend(bytes(bytes_in))

            def serialize_bool(self, value):
                self.serialize_number(U8_BYTES, False, 1 if value else 0)

            def serialize_number(self, n_bytes, signed, value):
                if n_bytes == U8_BYTES:
                    self.bytes.extend(value.to_bytes(1, byteorder="little", signed=signed))
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

pub fn gen_ser_functions(bindings: impl Iterator<Item = Container>) -> Tokens {
    bindings
        .map(gen_ser_function_for_type)
        .join_with_empty_line()
}

fn gen_ser_function_for_type(container: Container) -> impl FormatInto<Python> {
    let container_ident =
        ContainerIdentifierBuilder::new(container.path.clone().into_buf(), container.name).build();
    let body = container.r#type.gen_ser_body((&container).into());

    Function::new_untyped(
        quote!(serialize_$container_ident),
        function_args!("s", PYTHON_OBJECT_VARIABLE),
        body,
    )
}

pub fn gen_serialize_func(
    containers: impl Iterator<Item = Container> + Clone,
    runtime_type_checks: bool,
) -> Tokens {
    let all_bindings = containers
        .clone()
        .map(|d| quote!($(d.name)))
        .collect::<Vec<_>>();

    let type_check = if all_bindings.len() == 1 {
        quote!($(all_bindings.first().unwrap()))
    } else {
        quote!(Union[$(containers.clone().map(|container|
            quote!($(ContainerFullQualifiedTypeBuilder::from(&container)
                .build()))).join_with_comma())])
    };

    let ser_switch = containers
        .map(|t| gen_ser_case(t, runtime_type_checks))
        .map(|(condition, body)| (Some(condition), body))
        .chain([(
            None,
            quote!(raise TypeError("{} not serializable".format(type(value)))),
        )])
        .join_if_branched();

    let mut tokens = Tokens::new();
    if runtime_type_checks {
        let mut import_registry = ImportRegistry::new("".to_owned());
        import_registry.push(Package::Relative("runtime_checks".into()), ImportItem::All);

        quote_in!(tokens=> $import_registry);
        tokens.push();
    }

    let ser_func = Function::new(
        "serialize",
        function_args![("value", type_check)],
        quote! {
            s = Serializer()

            $ser_switch

            return s.finish()
        },
        "bytes",
    );

    tokens.append(ser_func);

    tokens
}

fn gen_ser_case(container: Container, runtime_type_checks: bool) -> (Tokens, Tokens) {
    let case_str = ContainerFullQualifiedTypeBuilder::from(&container).build();
    let container_ident = ContainerIdentifierBuilder::from(&container).build();

    let case_body = {
        let mut tokens = Tokens::new();

        if runtime_type_checks {
            quote_in!(tokens=> assert_$(&container_ident)(value));
            tokens.push();
        }
        quote_in!(tokens=> serialize_$container_ident(s, value));

        tokens
    };

    (quote!(isinstance(value, $case_str)), case_body)
}
