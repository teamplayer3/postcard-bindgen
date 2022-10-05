mod des;
mod ser;
pub mod type_checking;

pub use des::gen_deserialize_func;
pub use ser::gen_serialize_func;

use genco::{prelude::js::Tokens, quote};

use crate::registry::{BindingType, EnumType, StructType, TupleStructType};

const JS_ENUM_VARIANT_KEY: &str = "tag";
const JS_ENUM_VARIANT_VALUE: &str = "value";

pub fn gen_ser_des_classes() -> Tokens {
    quote!(
        const U8_BYTES = 1, U16_BYTES = 2, U32_BYTES = 4, U64_BYTES = 8, U128_BYTES = 16;
        const de_zig_zag_signed = t => void 0 === t ? void 0 : t >> 1 ^ -(1 & t), zig_zag = (t, e) => e << 1 ^ e >> 8 * t, varint_max = t => Math.floor((8 * t + 7) / 7), max_val = t => Math.pow(2, 8 * t) - 1;
        class Deserializer{bytes;constructor(t){this.bytes=Array.from(t)}pop_next=()=>{let t=this.bytes.shift();if(void 0===t)throw "input buffer too small";return t};pop_n=t=>{let e=[];for(let i=0;i<t;i++)e.push(this.bytes.shift());return e};get_uint8=()=>this.pop_next();try_take=t=>{let e=0,i=max_val(t);for(let s=0;s<varint_max(t);s++){let r=this.pop_next(),h=127&r;if(e|=h<<7*s,(128&r)==0){if(!(h>i))return e;throw "Bad Variant"}}};deserialize_bool=()=>{let t=this.pop_next();return void 0===t?void 0:t>0};deserialize_number=(t,e)=>{if(t===U8_BYTES)return this.get_uint8();if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES)return e?de_zig_zag_signed(this.try_take(U16_BYTES)):this.try_take(U16_BYTES);throw "byte count not supported"};deserialize_string=()=>{let t=this.pop_n(this.try_take(U32_BYTES));return String.fromCharCode(...t)};deserialize_array=t=>[...Array(this.try_take(U32_BYTES))].map(()=>t(this))}
        class Serializer{bytes=[];finish=()=>this.bytes;serialize_bool=t=>this.serialize_number(U8_BYTES,!1,t?1:0);serialize_number=(t,e,i)=>{if(t===U8_BYTES)this.bytes.push(i);else if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES){let s=e?this.varint(t,zig_zag(t,i)):this.varint(t,i);this.push_n(s)}else throw "byte count not supported"};serialize_string=t=>{this.push_n(this.varint(U32_BYTES,t.length));let e=[];for(let i of t)e.push(i.charCodeAt(0));this.push_n(e)};serialize_array=(t,e)=>{this.push_n(this.varint(U32_BYTES,e.length)),e.forEach(e=>t(this,e))};push_n=t=>{t.forEach(t=>this.bytes.push(t))};varint=(t,e)=>{let i=e,s=[];for(let r=0;r<varint_max(t);r++){if(s.push(255&i),i<128)return s;s[r]|=128,i>>=7}}}
    )
}

pub fn gen_ser_des_functions(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(|binding| match binding {
        BindingType::Enum(ty) => generate_js_enum(ty),
        BindingType::Struct(ty) => generate_js_object(ty),
        BindingType::TupleStruct(ty) => generate_js_object_tuple(ty),
        BindingType::UnitStruct(ty) => generate_js_obj_unit(&ty.name),
    }))
}

fn generate_js_obj_unit(name: impl AsRef<str>) -> Tokens {
    quote! {
        $(ser::strukt::gen_function(name.as_ref(), &[]))
        $(des::strukt::gen_function(name, &[]))
    }
}

fn generate_js_object(ty: &StructType) -> Tokens {
    let obj_name = &ty.name;
    quote! {
        $(ser::strukt::gen_function(obj_name, &ty.fields))
        $(des::strukt::gen_function(obj_name, &ty.fields))
    }
}

fn generate_js_object_tuple(ty: &TupleStructType) -> Tokens {
    let obj_name = &ty.name;
    quote! {
        $(ser::tuple_struct::gen_function(obj_name, &ty.fields))
        $(des::tuple_struct::gen_function(obj_name, &ty.fields))
    }
}

fn generate_js_enum(ty: &EnumType) -> Tokens {
    let obj_name = &ty.name;
    quote! {
        $(ser::enum_ty::gen_function(obj_name, &ty.variants))
        $(des::enum_ty::gen_function(obj_name, &ty.variants))
    }
}

fn line_brake_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join ($['\n']) => $part))
}

#[allow(unused)]
fn joined_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    parts.into_iter().fold(Tokens::new(), |mut res, p| {
        res.append(p);
        res
    })
}

fn semicolon_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join (;) => $part))
}

fn comma_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join (,) => $part))
}
