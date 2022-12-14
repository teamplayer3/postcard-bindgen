#[cfg(test)]
mod test;

use genco::{prelude::js::Tokens, quote, tokens::quoted};

use crate::{code_gen::utils::line_brake_chain, registry::BindingType, utils::StrExt};

use super::{
    generateable::binding_tys::BindingTypeGenerateable, utils::semicolon_chain, JS_OBJECT_VARIABLE,
};

pub fn gen_ser_des_classes() -> Tokens {
    quote!(
        const U8_BYTES = 1, U16_BYTES = 2, U32_BYTES = 4, U64_BYTES = 8, U128_BYTES = 16;
        const de_zig_zag_signed = t => void 0 === t ? void 0 : t >> 1 ^ -(1 & t), zig_zag = (t, e) => e << 1 ^ e >> 8 * t, varint_max = t => Math.floor((8 * t + 7) / 7), max_val = t => Math.pow(2, 8 * t) - 1;
        class Deserializer{bytes;constructor(t){this.bytes=Array.from(t)}pop_next=()=>{let t=this.bytes.shift();if(void 0===t)throw "input buffer too small";return t};pop_n=t=>{let e=[];for(let i=0;i<t;i++)e.push(this.bytes.shift());return e};get_uint8=()=>this.pop_next();try_take=t=>{let e=0,i=max_val(t);for(let s=0;s<varint_max(t);s++){let r=this.pop_next(),h=127&r;if(e|=h<<7*s,(128&r)==0){if(!(h>i))return e;throw "Bad Variant"}}};deserialize_bool=()=>{let t=this.pop_next();return void 0===t?void 0:t>0};deserialize_number=(t,e)=>{if(t===U8_BYTES)return this.get_uint8();if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES)return e?de_zig_zag_signed(this.try_take(U16_BYTES)):this.try_take(U16_BYTES);throw "byte count not supported"};deserialize_string=()=>{let t=this.pop_n(this.try_take(U32_BYTES));return String.fromCharCode(...t)};deserialize_array=t=>[...Array(this.try_take(U32_BYTES))].map(()=>t(this));deserialize_string_key_map=t=>[...Array(this.try_take(U32_BYTES))].reduce(e=>(e[this.deserialize_string()]=t(this),e),{});deserialize_map=t=>[...Array(this.try_take(U32_BYTES))].reduce(e=>{let i=t(this);return e.set(i[0],i[1]),e},new Map)}
        class Serializer{bytes=[];finish=()=>this.bytes;serialize_bool=t=>this.serialize_number(U8_BYTES,!1,t?1:0);serialize_number=(t,e,i)=>{if(t===U8_BYTES)this.bytes.push(i);else if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES){let s=e?this.varint(t,zig_zag(t,i)):this.varint(t,i);this.push_n(s)}else throw "byte count not supported"};serialize_string=t=>{this.push_n(this.varint(U32_BYTES,t.length));let e=[];for(let i of t)e.push(i.charCodeAt(0));this.push_n(e)};serialize_array=(t,e)=>{this.push_n(this.varint(U32_BYTES,e.length)),e.forEach(e=>t(this,e))};serialize_string_key_map=(t,e)=>{let i=Object.entries(e);this.push_n(this.varint(U32_BYTES,i.length)),i.forEach(([e,i])=>{this.serialize_string(e),t(this,i)})};serialize_map=(t,e)=>{this.push_n(this.varint(U32_BYTES,e.size)),e.forEach((e,i)=>t(this,i,e))};push_n=t=>{t.forEach(t=>this.bytes.push(t))};push_n=t=>{t.forEach(t=>this.bytes.push(t))};varint=(t,e)=>{let i=e,s=[];for(let r=0;r<varint_max(t);r++){if(s.push(255&i),i<128)return s;s[r]|=128,i>>=7}}}
    )
}

pub fn gen_ser_des_functions(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(gen_ser_des_funcs))
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
