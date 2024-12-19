use genco::{
    lang::JavaScript,
    quote,
    tokens::{quoted, FormatInto},
};

use crate::{
    code_gen::{
        js::{
            generateable::container::BindingTypeGenerateable, Function, Tokens, JS_OBJECT_VARIABLE,
        },
        utils::{ContainerFullQualifiedTypeBuilder, ContainerIdentifierBuilder, TokensIterExt},
    },
    function_args,
    registry::Container,
};

use super::{Case, DefaultCase, ExportRegistry, SwitchCase};

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

pub fn gen_ser_functions(bindings: impl Iterator<Item = Container>) -> Tokens {
    bindings
        .map(gen_ser_function_for_type)
        .join_with_empty_line()
}

fn gen_ser_function_for_type(container: Container) -> impl FormatInto<JavaScript> {
    let container_ident = ContainerIdentifierBuilder::from(&container).build();
    let ser_body = container.r#type.gen_ser_body();

    Function::new_untyped(
        quote!(serialize_$container_ident),
        function_args![quote!(s), JS_OBJECT_VARIABLE],
        ser_body,
    )
}

pub fn gen_serialize_func(
    defines: impl Iterator<Item = Container>,
    runtime_type_checks: bool,
    export_registry: &mut ExportRegistry,
) -> impl FormatInto<JavaScript> {
    let mut switch_case = SwitchCase::new("type");
    switch_case.extend_cases(defines.map(|d| gen_ser_case(d, runtime_type_checks)));
    switch_case.default_case(DefaultCase::new_without_break(
        quote!(throw "type not implemented";),
    ));

    export_registry.push("serialize");

    Function::new_untyped(
        "serialize",
        function_args!["type", "value"],
        quote! {
            if (!(typeof type === "string")) {
                throw "type must be a string";
            }
            const s = new Serializer();
            $switch_case
            return s.finish();
        },
    )
}

fn gen_ser_case(container: Container, runtime_type_checks: bool) -> Case {
    let full_qualified = ContainerFullQualifiedTypeBuilder::from(&container).build();
    let container_ident = ContainerIdentifierBuilder::from(&container).build();
    let body = if runtime_type_checks {
        quote! {
            if (is_$(container_ident.as_str())(value)) {
                serialize_$(container_ident)(s, value);
            } else {
                throw "value has wrong format";
            }
        }
    } else {
        quote! {
            serialize_$(container_ident)(s, value);
        }
    };

    Case::new(quoted(full_qualified), body)
}
