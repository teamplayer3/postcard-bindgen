pub mod des;
pub mod ser;

use genco::{prelude::js::Tokens, quote};

use crate::ArchPointerLen;

pub fn gen_ser_des_classes(pointer_type: ArchPointerLen) -> Tokens {
    quote!(
        const U8_BYTES = 1, U16_BYTES = 2, U32_BYTES = 4, U64_BYTES = 8, U128_BYTES = 16, USIZE_BYTES = $(pointer_type.into_bytes());
        const de_zig_zag_signed = t => void 0 === t ? void 0 : t >> 1 ^ -(1 & t), zig_zag = (t, e) => e << 1 ^ e >> 8 * t, varint_max = t => Math.floor((8 * t + 7) / 7), max_val = t => Math.pow(2, 8 * t) - 1;
        class Deserializer{bytes;constructor(t){this.bytes=Array.from(t)}pop_next=()=>{let t=this.bytes.shift();if(void 0===t)throw "input buffer too small";return t};pop_n=t=>{let e=Array(t);for(let i=0;i<t;i++)e.push(this.bytes.shift());return e};get_uint8=()=>this.pop_next();try_take=t=>{let e=0,i=max_val(t);for(let r=0;r<varint_max(t);r++){let s=this.pop_next(),h=127&s;if(e|=h<<7*r,(128&s)==0){if(!(h>i))return e;throw "Bad Variant"}}};deserialize_bool=()=>{let t=this.pop_next();return void 0===t?void 0:t>0};deserialize_number=(t,e)=>{if(t===U8_BYTES)return this.get_uint8();if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES)return e?de_zig_zag_signed(this.try_take(U16_BYTES)):this.try_take(U16_BYTES);throw "byte count not supported"};deserialize_string=()=>{let t=this.pop_n(this.try_take(U32_BYTES));return String.fromCharCode(...t)};deserialize_array=(t,e)=>[...Array(this.try_take(U32_BYTES))].map(()=>this.deserialize_number(t,e))}
        class Serializer{bytes=[];finish=()=>this.bytes;serialize_bool=t=>this.serialize_number(U8_BYTES,!1,t?1:0);serialize_number=(t,e,i)=>{if(t===U8_BYTES)this.bytes.push(i);else if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES){let r=e?this.varint(t,zig_zag(t,i)):this.varint(t,i);this.push_n(r)}else throw "byte count not supported"};serialize_string=t=>{this.push_n(this.varint(U32_BYTES,t.length));let e=Array(t.length);for(let i=0;i<t.length;i++)e.push(t.charCodeAt(i));this.push_n(e)};serialize_array=(t,e,i)=>{this.push_n(this.varint(U32_BYTES,i.length)),i.forEach(i=>this.serialize_number(t,e,i))};push_n=t=>{t.forEach(t=>this.bytes.push(t))};varint=(t,e)=>{let i=e,r=[];for(let s=0;s<varint_max(t);s++){if(r.push(255&i),i<128)return r;r[s]|=128,i>>=7}}}
    )
}
