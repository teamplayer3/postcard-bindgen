import { serialize, deserialize } from "./js_export.js";
import fs from "fs";

const d = {
    a: 22,
    b: {
        key: "D",
        inner: {
            a: [234, 224],
            b: [123, [123, 431, 123232], "Hello"]
        }
    },
    c: {},
    d: [234, 213, 123]
}

console.log(d.b.inner.b)

const bytes = serialize("D", d)
console.log(bytes)

const deser = deserialize("D", bytes)
console.log(deser)

const bytes_file = `${process.cwd()}/serialized.bytes`
const loaded_bytes = fs.readFileSync(bytes_file)
const rust_des = deserialize("D", loaded_bytes);
console.log(rust_des)