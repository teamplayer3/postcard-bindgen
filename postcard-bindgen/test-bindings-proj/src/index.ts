import { readFileSync } from "fs";
import { deserialize, serialize } from "js-test-bindings";


const map = new Map()
map.set(234, 21)
const bytes = serialize("B", [234, [234], "Hello", map])
console.log(bytes)
const b = deserialize("B", bytes)
console.log(b)

// test namespaces
const bytes_e_e = serialize("e.E",  [234, [21]]);
console.log(bytes_e_e)
const e_e = deserialize("e.E", bytes_e_e)
console.log(e_e)

const bytes_file = `${process.cwd()}/../serialized.bytes`
const loaded_bytes = readFileSync(bytes_file)
const rust_des = deserialize("D", [...loaded_bytes]);
console.log(rust_des)