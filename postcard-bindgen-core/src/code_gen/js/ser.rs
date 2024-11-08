use genco::quote;

use crate::{
    code_gen::{
        js::{generateable::container::BindingTypeGenerateable, Tokens, JS_OBJECT_VARIABLE},
        utils::{ContainerIdentifierBuilder, TokensIterExt},
    },
    registry::Container,
};

use super::utils::ContainerCaseTypeBuilder;

pub fn gen_serializer_code() -> Tokens {
    quote! {
        class Serializer {
            constructor() { this.bytes = [] }
            finish = () => this.bytes
            push_n = (bytes) => bytes.forEach((byte) => this.bytes.push(byte))
            serialize_bool = (value) => this.serialize_number(U8_BYTES, false, value ? 1 : 0)
            serialize_number = (n_bytes, signed, value) => { if (n_bytes === U8_BYTES) { this.bytes.push(value) } else if (n_bytes === U16_BYTES || n_bytes === U32_BYTES || n_bytes === U64_BYTES || n_bytes === U128_BYTES) { const value_b = BigInt(value), buffer = signed ? varint(n_bytes, zig_zag(n_bytes, value_b)) : varint(n_bytes, value_b); this.push_n(buffer) } else { throw "byte count not supported" } }
            serialize_number_float = (n_bytes, value) => { const b_buffer = new ArrayBuffer(n_bytes), b_view = new DataView(b_buffer); if (n_bytes === U32_BYTES) { b_view.setFloat32(0, value, true) } else if (n_bytes === U64_BYTES) { b_view.setFloat64(0, value, true) } else { throw "byte count not supported" } this.push_n(new Uint8Array(b_buffer)) }
            serialize_string = (str) => { this.push_n(varint(U32_BYTES, str.length)); const bytes = []; for (const c of str) { bytes.push(c.charCodeAt(0)) } this.push_n(bytes) }
            serialize_array = (ser, array, len) => { if (len == undefined) this.push_n(varint(U32_BYTES, array.length)); array.slice(0, len != undefined ? len : array.length).forEach((v) => ser(this, v)) }
            serialize_string_key_map = (ser, obj) => { const entries = Object.entries(obj); this.push_n(varint(U32_BYTES, entries.length)); entries.forEach(([i, v]) => { this.serialize_string(i); ser(this, v) }) }
            serialize_map = (ser, map) => { this.push_n(varint(U32_BYTES, map.size)); map.forEach((v, k) => ser(this, k, v)) }
        }

    }
}

pub fn gen_ser_functions(bindings: impl AsRef<[Container]>) -> Tokens {
    bindings
        .as_ref()
        .iter()
        .map(gen_ser_function_for_type)
        .join_with_line_breaks()
}

fn gen_ser_function_for_type(container: &Container) -> Tokens {
    let container_ident = ContainerIdentifierBuilder::new(&container.path, container.name).build();
    let ser_body = container.r#type.gen_ser_body();
    quote! {
        const serialize_$(&container_ident) = (s, $JS_OBJECT_VARIABLE) => { $ser_body }
    }
}

pub fn gen_serialize_func(defines: impl AsRef<[Container]>, runtime_type_checks: bool) -> Tokens {
    let body = defines
        .as_ref()
        .iter()
        .map(|d| gen_ser_case(d, runtime_type_checks))
        .join_with_semicolon();
    quote!(
        module.exports.serialize = (type, value) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const s = new Serializer()
            switch (type) { $body }
            return s.finish()
        }
    )
}

fn gen_ser_case(container: &Container, runtime_type_checks: bool) -> Tokens {
    let case_str = ContainerCaseTypeBuilder::new(&container.path, container.name).build();
    let container_ident = ContainerIdentifierBuilder::new(&container.path, container.name).build();
    if runtime_type_checks {
        quote!(case $case_str: if (is_$(container_ident.as_str())(value)) { serialize_$(container_ident)(s, value) } else throw "value has wrong format"; break)
    } else {
        quote!(case $case_str: serialize_$(container_ident)(s, value); break)
    }
}
