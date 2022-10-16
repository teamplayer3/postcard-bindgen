import { readFileSync } from "fs";
import { deserialize, serialize } from "test-postcard-bindings";


const bytes = serialize("B", [234, [234], "Hello"])
console.log(bytes)
const b = deserialize("B", bytes)
console.log(b)

const bytes_file = `${process.cwd()}/../serialized.bytes`
const loaded_bytes = readFileSync(bytes_file)
const rust_des = deserialize("D", [...loaded_bytes]);
console.log(rust_des)