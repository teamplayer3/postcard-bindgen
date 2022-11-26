const U8_BYTES = 1, U16_BYTES = 2, U32_BYTES = 4, U64_BYTES = 8, U128_BYTES = 16;
const de_zig_zag_signed = t => void 0 === t ? void 0 : t >> 1 ^ -(1 & t), zig_zag = (t, e) => e << 1 ^ e >> 8 * t, varint_max = t => Math.floor((8 * t + 7) / 7), max_val = t => Math.pow(2, 8 * t) - 1;
class Deserializer{bytes;constructor(t){this.bytes=Array.from(t)}pop_next=()=>{let t=this.bytes.shift();if(void 0===t)throw "input buffer too small";return t};pop_n=t=>{let e=[];for(let i=0;i<t;i++)e.push(this.bytes.shift());return e};get_uint8=()=>this.pop_next();try_take=t=>{let e=0,i=max_val(t);for(let s=0;s<varint_max(t);s++){let r=this.pop_next(),h=127&r;if(e|=h<<7*s,(128&r)==0){if(!(h>i))return e;throw "Bad Variant"}}};deserialize_bool=()=>{let t=this.pop_next();return void 0===t?void 0:t>0};deserialize_number=(t,e)=>{if(t===U8_BYTES)return this.get_uint8();if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES)return e?de_zig_zag_signed(this.try_take(U16_BYTES)):this.try_take(U16_BYTES);throw "byte count not supported"};deserialize_string=()=>{let t=this.pop_n(this.try_take(U32_BYTES));return String.fromCharCode(...t)};deserialize_array=t=>[...Array(this.try_take(U32_BYTES))].map(()=>t(this));deserialize_string_key_map=t=>[...Array(this.try_take(U32_BYTES))].reduce(e=>(e[this.deserialize_string()]=t(this),e),{});deserialize_map=t=>[...Array(this.try_take(U32_BYTES))].reduce(e=>{let i=t(this);return e.set(i[0],i[1]),e},new Map)}
class Serializer{bytes=[];finish=()=>this.bytes;serialize_bool=t=>this.serialize_number(U8_BYTES,!1,t?1:0);serialize_number=(t,e,i)=>{if(t===U8_BYTES)this.bytes.push(i);else if(t===U16_BYTES||t===U32_BYTES||t===U64_BYTES||t===U128_BYTES){let s=e?this.varint(t,zig_zag(t,i)):this.varint(t,i);this.push_n(s)}else throw "byte count not supported"};serialize_string=t=>{this.push_n(this.varint(U32_BYTES,t.length));let e=[];for(let i of t)e.push(i.charCodeAt(0));this.push_n(e)};serialize_array=(t,e)=>{this.push_n(this.varint(U32_BYTES,e.length)),e.forEach(e=>t(this,e))};serialize_string_key_map=(t,e)=>{let i=Object.entries(e);this.push_n(this.varint(U32_BYTES,i.length)),i.forEach(([e,i])=>{this.serialize_string(e),t(this,i)})};serialize_map=(t,e)=>{this.push_n(this.varint(U32_BYTES,e.size)),e.forEach((e,i)=>t(this,i,e))};push_n=t=>{t.forEach(t=>this.bytes.push(t))};push_n=t=>{t.forEach(t=>this.bytes.push(t))};varint=(t,e)=>{let i=e,s=[];for(let r=0;r<varint_max(t);r++){if(s.push(255&i),i<128)return s;s[r]|=128,i>>=7}}}
const serialize_PROTOCOL = (s, v) => { serialize_PACKET(s, v.packet) }
const deserialize_PROTOCOL = (d) => ({ packet: deserialize_PACKET(d) })
const serialize_PACKET = (s, v) => { switch (v.tag) { case "A1": s.serialize_number(U32_BYTES, false, 0); serialize_A_1_META(s, v.value); break } }
const deserialize_PACKET = (d) => { switch (d.deserialize_number(U32_BYTES, false)) { case 0: return { tag: "A1" , value: deserialize_A_1_META(d) }; default: throw "variant not implemented" } }
const serialize_A_1_META = (s, v) => { s.serialize_string(v.name); s.serialize_number(U16_BYTES, false, v.version); s.serialize_array((s, v) => s.serialize_number(U8_BYTES, false, v), v.payload) }
const deserialize_A_1_META = (d) => ({ name: d.deserialize_string(), version: d.deserialize_number(U16_BYTES, false), payload: d.deserialize_array(() => d.deserialize_number(U8_BYTES, false)) })
const is_PROTOCOL = (v) => (typeof v === "object" && is_PACKET(v.packet))
const is_PACKET = (v) => ((typeof v === "object" && "tag" in v && "value" in v && (v.tag === "A1" && is_A_1_META(v.value))))
const is_A_1_META = (v) => (typeof v === "object" && typeof v.name === "string" && typeof v.version === "number" && Array.isArray(v.payload))
module.exports.serialize = (type, value) => {
    if (!(typeof type === "string")) {
        throw "type must be a string"
    }
    const s = new Serializer()
    switch (type) { case "Protocol": if (is_PROTOCOL(value)) { serialize_PROTOCOL(s, value) } else throw "value has wrong format"; break; case "Packet": if (is_PACKET(value)) { serialize_PACKET(s, value) } else throw "value has wrong format"; break; case "A1Meta": if (is_A_1_META(value)) { serialize_A_1_META(s, value) } else throw "value has wrong format"; break }
    return s.finish()
}
module.exports.deserialize = (type, bytes) => {
    if (!(typeof type === "string")) {
        throw "type must be a string"
    }
    const d = new Deserializer(bytes)
    switch (type) { case "Protocol": return deserialize_PROTOCOL(d); case "Packet": return deserialize_PACKET(d); case "A1Meta": return deserialize_A_1_META(d) }
}
