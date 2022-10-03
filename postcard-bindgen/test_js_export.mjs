import { serialize, deserialize } from "./js_export.js";
import fs from "fs";

const test = {
    name: 23,
    other: 24322
}

const d = {
    a: 22,
    b: {
        key: "D",
        inner: {
            a: 234,
            b: [123]
        }
    },
    c: {}
}

const bytes = serialize("D", d)
console.log(bytes)

const deser = deserialize("D", bytes)
console.log(deser)

const bytes_file = `${process.cwd()}/serialized.bytes`
const loaded_bytes = fs.readFileSync(bytes_file)
console.log(loaded_bytes)
const rust_des = deserialize("D", loaded_bytes);
console.log(rust_des)