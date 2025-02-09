use genco::{
    lang::JavaScript,
    quote,
    tokens::{quoted, FormatInto},
};

use crate::{
    code_gen::{
        js::{generateable::container::BindingTypeGenerateable, Tokens},
        utils::{ContainerFullQualifiedTypeBuilder, ContainerIdentifierBuilder, TokensIterExt},
    },
    function_args,
    registry::Container,
};

use super::{Case, DefaultCase, ExportRegistry, Function, SwitchCase};

pub fn gen_deserializer_code() -> Tokens {
    quote! {
        class Deserializer {
            constructor(bytes_in) { this.bytes = Array.from(bytes_in) }
            pop_next = () => { const next = this.bytes.shift(); if (next === undefined) { throw "input buffer too small" } return next }
            pop_n = (n) => { const bytes = Array(); for (let i = 0; i < n; i++) { bytes.push(this.bytes.shift()) } return bytes }
            get_int8 = (signed) => signed ? new Int8Array([this.pop_next()])[0] : this.pop_next();
            try_take = (n_bytes) => { let out = 0n, v_max = varint_max(n_bytes); for (let i = 0; i < v_max; i++) { const val = this.pop_next(), carry = BigInt(val & 0x7F); out |= carry << BigInt(7 * i); if ((val & 0x80) === 0) { if (i === v_max - 1 && val > max_of_last_byte(n_bytes)) { throw "Bad Variant" } else return out } } throw "Bad Variant"; }
            deserialize_bool = () => { const byte = this.pop_next(); return byte === undefined ? undefined : byte > 0 ? true : false }
            deserialize_number = (n_bytes, signed) => { if (n_bytes === U8_BYTES) { return this.get_int8(signed) } else if (n_bytes === U16_BYTES || n_bytes === U32_BYTES || n_bytes === U64_BYTES || n_bytes === U128_BYTES) { const val = this.try_take(n_bytes); return to_number_if_safe(signed ? de_zig_zag_signed(val) : val) } else { throw "byte count not supported" } }
            deserialize_number_float = (n_bytes) => { const b_buffer = new ArrayBuffer(n_bytes), b_view = new DataView(b_buffer); this.pop_n(n_bytes).forEach((b, i) => b_view.setUint8(i, b)); if (n_bytes === U32_BYTES) { return b_view.getFloat32(0, true) } else if (n_bytes === U64_BYTES) { return b_view.getFloat64(0, true) } else { throw "byte count not supported" } }
            deserialize_string = () => { const str = this.pop_n(Number(this.try_take(U32_BYTES))); return String.fromCharCode(...str) }
            deserialize_array = (des, len) => Array.from({length: len === undefined ? Number(this.try_take(U32_BYTES)) : len}, (v, i) => des(this))
            deserialize_string_key_map = (des) => { return [...Array(Number(this.try_take(U32_BYTES)))].reduce((prev) => { prev[this.deserialize_string()] = des(this); return prev }, {}) }
            deserialize_map = (des) => { return [...Array(Number(this.try_take(U32_BYTES)))].reduce((prev) => { const d = des(this); prev.set(d[0], d[1]); return prev }, new Map()) }
        }
    }
}

pub fn gen_des_functions(bindings: impl Iterator<Item = Container>) -> Tokens {
    bindings
        .map(gen_des_function_for_type)
        .join_with_empty_line()
}

fn gen_des_function_for_type(container: Container) -> impl FormatInto<JavaScript> {
    let container_ident = ContainerIdentifierBuilder::from(&container).build();
    let des_body = container.r#type.gen_des_body();

    Function::new_untyped(
        quote!(deserialize_$container_ident),
        function_args![quote!(d)],
        des_body,
    )
}

pub fn gen_deserialize_func(
    defines: impl Iterator<Item = Container>,
    export_registry: &mut ExportRegistry,
) -> impl FormatInto<JavaScript> {
    let mut switch_case = SwitchCase::new("type");
    switch_case.extend_cases(defines.map(gen_des_case));
    switch_case.default_case(DefaultCase::new_without_break(
        quote!(throw "type not implemented";),
    ));

    let body = quote! {
        if (!(typeof type === "string")) {
            throw "type must be a string";
        }
        const d = new Deserializer(bytes);
        $switch_case
    };

    export_registry.push("deserialize");

    Function::new_untyped("deserialize", function_args!("type", "bytes"), body)
}

fn gen_des_case(container: Container) -> Case {
    let fully_qualified = ContainerFullQualifiedTypeBuilder::from(&container).build();
    let container_ident = ContainerIdentifierBuilder::from(&container).build();

    Case::new_without_break(
        quoted(fully_qualified),
        quote!(return deserialize_$container_ident(d);),
    )
}
