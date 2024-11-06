use genco::{prelude::python::Tokens, quote};

use crate::{
    code_gen::{
        python::generateable::container::BindingTypeGenerateable,
        utils::{TokensBranchedIterExt, TokensIterExt},
    },
    registry::BindingType,
    // utils::StrExt,
};

pub fn gen_deserializer_code() -> Tokens {
    quote! {
        import struct

        from .util import *

        class Deserializer:
            def __init__(self, bytes_in):
                self.bytes = list(bytes_in)

            def pop_next(self):
                if not self.bytes:
                    raise Exception("input buffer too small")
                return self.bytes.pop(0)

            def pop_n(self, n):
                bytes_out = []
                for _ in range(n):
                    bytes_out.append(self.pop_next())
                return bytes_out

            def get_uint8(self):
                return self.pop_next()

            def try_take(self, n_bytes):
                out = 0
                v_max = varint_max(n_bytes)
                for i in range(v_max):
                    val = self.pop_next()
                    carry = val & 0x7F
                    out |= carry << (7 * i)
                    if (val & 0x80) == 0:
                        if i == v_max - 1 and val > max_of_last_byte(n_bytes):
                            raise Exception("Bad Variant")
                        else:
                            return out
                raise Exception("Bad Variant")

            def deserialize_bool(self):
                byte = self.pop_next()
                return byte is not None and byte > 0

            def deserialize_number(self, n_bytes, signed):
                if n_bytes == U8_BYTES:
                    return self.get_uint8()
                elif n_bytes in {U16_BYTES, U32_BYTES, U64_BYTES, U128_BYTES}:
                    val = self.try_take(n_bytes)
                    return to_number_if_safe(de_zig_zag_signed(val) if signed else val)
                else:
                    raise Exception("byte count not supported")

            def deserialize_number_float(self, n_bytes):
                b_buffer = bytes(self.pop_n(n_bytes))
                if n_bytes == U32_BYTES:
                    return struct.unpack("<f", b_buffer)[0]
                elif n_bytes == U64_BYTES:
                    return struct.unpack("<d", b_buffer)[0]
                else:
                    raise Exception("byte count not supported")

            def deserialize_string(self):
                str_len = self.try_take(U32_BYTES)
                str_bytes = self.pop_n(str_len)
                return "".join(chr(b) for b in str_bytes)

            def deserialize_array(self, des, length = None):
                return [des(self) for _ in range(self.try_take(U32_BYTES) if length is None else length)]

            def deserialize_map(self, des):
                return {key: value for key, value in (des(self) for _ in range(self.try_take(U32_BYTES)))}
    }
}

pub fn gen_des_functions(bindings: impl AsRef<[BindingType]>) -> Tokens {
    bindings
        .as_ref()
        .iter()
        .map(gen_des_function_for_type)
        .join_with_empty_line()
}

fn gen_des_function_for_type(binding_type: &BindingType) -> Tokens {
    // let obj_name = binding_type.inner_name().to_obj_identifier();
    let obj_name = String::from(binding_type.inner_name());
    let des_body = binding_type.gen_des_body();
    quote! {
        def deserialize_$(&obj_name)(d) -> $obj_name:
            $des_body
    }
}

pub fn gen_deserialize_func(tys: impl AsRef<[BindingType]>) -> Tokens {
    let all_bindings = tys
        .as_ref()
        .iter()
        .map(|d| quote!($(d.inner_name())))
        .collect::<Vec<_>>();

    let mut obj_type_types = all_bindings.iter().map(|d| quote!($d));

    let obj_type_type = if all_bindings.len() == 1 {
        quote!(T = $(obj_type_types.next().unwrap()))
    } else {
        quote!(T = TypeVar("T", $(obj_type_types.join_with_comma())))
    };

    let des_switch = tys
        .as_ref()
        .iter()
        .map(gen_des_case)
        .map(|(condition, body)| (Some(condition), body))
        .chain([(
            None,
            quote!(raise TypeError("{} not deserializable".format(obj_type))),
        )])
        .join_if_branched();

    quote! {
        $obj_type_type
        def deserialize(obj_type: Type[T], bytes: bytes) -> T:
            d = Deserializer(bytes)

            $des_switch
    }
}

fn gen_des_case(define: &BindingType) -> (Tokens, Tokens) {
    let name = define.inner_name();
    // let type_name = name.to_obj_identifier();
    let type_name = String::from(name);
    (
        quote!(obj_type is $(&type_name)),
        quote!(return cast(T, deserialize_$type_name(d))),
    )
}
