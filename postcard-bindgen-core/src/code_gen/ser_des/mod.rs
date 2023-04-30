#[cfg(test)]
mod test;

use genco::{prelude::js::Tokens, quote, tokens::quoted};

use crate::{code_gen::utils::line_break_chain, registry::BindingType, utils::StrExt};

use super::{
    generateable::container::BindingTypeGenerateable, utils::semicolon_chain, JS_OBJECT_VARIABLE,
};

pub fn gen_ser_des_classes() -> Tokens {
    quote!(
        const BITS_PER_BYTE = 8, BITS_PER_VARINT_BYTE = 7, U8_BYTES = 1, U16_BYTES = 2, U32_BYTES = 4, U64_BYTES = 8, U128_BYTES = 16

        const de_zig_zag_signed = (n) => (n >> 1n) ^ (-(n & 0b1n))
        const zig_zag = (n_bytes, n) => (n << 1n) ^ (n >> BigInt(n_bytes * BITS_PER_BYTE - 1))
        const varint_max = (n_bytes) => Math.floor((n_bytes * BITS_PER_BYTE + (BITS_PER_BYTE - 1)) / BITS_PER_VARINT_BYTE)
        const max_of_last_byte = (n_bytes) => (1 << (n_bytes * BITS_PER_BYTE) % 7) - 1
        const to_number_if_safe = (n) => Number.MAX_SAFE_INTEGER < ((n < 0n) ? -n : n) ? n : Number(n)
        const varint = (n_bytes, n) => { let value = BigInt(n), out = []; for (let i = 0; i < varint_max(n_bytes); i++) { out.push(Number(value & 0xFFn)); if (value < 128n) { return out } out[i] |= 0x80; value >>= 7n } }

        class Deserializer {
            constructor(bytes_in) { this.bytes = Array.from(bytes_in) }
            pop_next = () => { const next = this.bytes.shift(); if (next === undefined) { throw "input buffer too small" } return next }
            pop_n = (n) => { const bytes = Array(); for (let i = 0; i < n; i++) { bytes.push(this.bytes.shift()) } return bytes }
            get_uint8 = () => this.pop_next()
            try_take = (n_bytes) => { let out = 0n, v_max = varint_max(n_bytes); for (let i = 0; i < v_max; i++) { const val = this.pop_next(), carry = BigInt(val & 0x7F); out |= carry << BigInt(7 * i); if ((val & 0x80) === 0) { if (i === v_max - 1 && val > max_of_last_byte(n_bytes)) { throw "Bad Variant" } else return out } } throw "Bad Variant"; }
            deserialize_bool = () => { const byte = this.pop_next(); return byte === undefined ? undefined : byte > 0 ? true : false }
            deserialize_number = (n_bytes, signed) => { if (n_bytes === U8_BYTES) { return this.get_uint8() } else if (n_bytes === U16_BYTES || n_bytes === U32_BYTES || n_bytes === U64_BYTES || n_bytes === U128_BYTES) { const val = this.try_take(n_bytes); return to_number_if_safe(signed ? de_zig_zag_signed(val) : val) } else { throw "byte count not supported" } }
            deserialize_number_float = (n_bytes) => { const b_buffer = new ArrayBuffer(n_bytes), b_view = new DataView(b_buffer); this.pop_n(n_bytes).forEach((b, i) => b_view.setUint8(i, b)); if (n_bytes === U32_BYTES) { return b_view.getFloat32(0, true) } else if (n_bytes === U64_BYTES) { return b_view.getFloat64(0, true) } else { throw "byte count not supported" } }
            deserialize_string = () => { const str = this.pop_n(this.try_take(U32_BYTES)); return String.fromCharCode(...str) }
            deserialize_array = (des) => [...Array(this.try_take(U32_BYTES))].map(() => des(this))
            deserialize_string_key_map = (des) => { return [...Array(this.try_take(U32_BYTES))].reduce((prev) => { prev[this.deserialize_string()] = des(this); return prev }, {}) }
            deserialize_map = (des) => { return [...Array(this.try_take(U32_BYTES))].reduce((prev) => { const d = des(this); prev.set(d[0], d[1]); return prev }, new Map()) }
        }

        class Serializer {
            constructor() { this.bytes = [] }
            finish = () => this.bytes
            push_n = (bytes) => bytes.forEach((byte) => this.bytes.push(byte))
            serialize_bool = (value) => this.serialize_number(U8_BYTES, false, value ? 1 : 0)
            serialize_number = (n_bytes, signed, value) => { if (n_bytes === U8_BYTES) { this.bytes.push(value) } else if (n_bytes === U16_BYTES || n_bytes === U32_BYTES || n_bytes === U64_BYTES || n_bytes === U128_BYTES) { const value_b = BigInt(value), buffer = signed ? varint(n_bytes, zig_zag(n_bytes, value_b)) : varint(n_bytes, value_b); this.push_n(buffer) } else { throw "byte count not supported" } }
            serialize_number_float = (n_bytes, value) => { const b_buffer = new ArrayBuffer(n_bytes), b_view = new DataView(b_buffer); if (n_bytes === U32_BYTES) { b_view.setFloat32(0, value, true) } else if (n_bytes === U64_BYTES) { b_view.setFloat64(0, value, true) } else { throw "byte count not supported" } this.push_n(new Uint8Array(b_buffer)) }
            serialize_string = (str) => { this.push_n(varint(U32_BYTES, str.length)); const bytes = []; for (const c of str) { bytes.push(c.charCodeAt(0)) } this.push_n(bytes) }
            serialize_array = (ser, array) => { this.push_n(varint(U32_BYTES, array.length)); array.forEach((v) => ser(this, v)) }
            serialize_string_key_map = (ser, obj) => { const entries = Object.entries(obj); this.push_n(varint(U32_BYTES, entries.length)); entries.forEach(([i, v]) => { this.serialize_string(i); ser(this, v) }) }
            serialize_map = (ser, map) => { this.push_n(varint(U32_BYTES, map.size)); map.forEach((v, k) => ser(this, k, v)) }
        }

        const check_bounds = (n_bytes, signed, value) => { const max = BigInt(2 ** (n_bytes * BITS_PER_BYTE)), value_b = BigInt(value); if (signed) { const bounds = max / 2n; return value_b >= -bounds && value_b < bounds } else { return value_b < max && value_b >= 0 } }
    )
}

pub fn gen_ser_des_functions(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_break_chain(bindings.as_ref().iter().map(gen_ser_des_funcs))
}

pub fn gen_serialize_func(defines: impl AsRef<[BindingType]>) -> Tokens {
    let body = semicolon_chain(defines.as_ref().iter().map(gen_ser_case));
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

pub fn gen_deserialize_func(defines: impl AsRef<[BindingType]>) -> Tokens {
    let body = semicolon_chain(defines.as_ref().iter().map(gen_des_case));
    quote!(
        module.exports.deserialize = (type, bytes) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const d = new Deserializer(bytes)
            switch (type) { $body }
        }
    )
}

fn gen_ser_case(define: &BindingType) -> Tokens {
    let name = define.inner_name();
    let case_str = quoted(name);
    let type_name = name.to_obj_identifier();
    quote!(case $case_str: if (is_$(type_name.as_str())(value)) { serialize_$(type_name)(s, value) } else throw "value has wrong format"; break)
}

fn gen_des_case(define: &BindingType) -> Tokens {
    let name = define.inner_name();
    let case_str = quoted(name);
    let type_name = name.to_obj_identifier();
    quote!(case $case_str: return deserialize_$type_name(d))
}

fn gen_ser_des_funcs(binding_type: &BindingType) -> Tokens {
    let obj_name = binding_type.inner_name().to_obj_identifier();
    let ser_body = binding_type.gen_ser_body();
    let des_body = binding_type.gen_des_body();
    quote! {
        const serialize_$(&obj_name) = (s, $JS_OBJECT_VARIABLE) => { $ser_body }
        const deserialize_$obj_name = (d) => $des_body
    }
}
