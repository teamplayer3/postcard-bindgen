import { readFileSync } from "fs";
import { deserialize, serialize } from "js-test-bindings";


const map = new Map()
map.set(234, 21)
const bytes = serialize("B", [234, [234], "Hello", map])
console.log(bytes)
const b = deserialize("B", bytes)
console.log(b)

// test namespaces
const bytes_b_b = serialize("b.B", [2323]);
console.log(bytes_b_b)
const b_b = deserialize("b.B", bytes_b_b)
console.log(b_b)

const bytes_file = `${process.cwd()}/../serialized.bytes`
const loaded_bytes = readFileSync(bytes_file)
const rust_des = deserialize("D", [...loaded_bytes]);
console.log(rust_des)