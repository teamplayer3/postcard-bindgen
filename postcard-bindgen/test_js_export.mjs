// Before running this with `$node .\test_js_export.mj` run the rust example with 
// `$cargo run --example generate_bindings --features std,generating`.

import fs from "fs";
import { serialize, deserialize } from "./test-bindings/index.js"

const d = {
    a: 22,
    b: {
        tag: "D",
        value: {
            a: [234, 224],
            b: [123, [123, 431, 1232], "Hello", new Map([[12, 32]])],
            c: false
        }
    },
    c: {},
    d: [234, 213, 123],
    e: undefined,
    f: [12, 123],
    g: "hello from js",
    h: {
        start: 23,
        end: 45
    },
    i: {
        name: 23,
    },
    j: new Map([[12, 32]]),
    k: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    m: [25, "hello", [34, 34, 22]],
    n: true
}

const bytes = serialize("D", d)
console.log(bytes)

const deser = deserialize("D", bytes)
console.log(deser)

const bytes_file = `${process.cwd()}/serialized.bytes`
const loaded_bytes = fs.readFileSync(bytes_file)
const rust_des = deserialize("D", loaded_bytes);
console.log(rust_des)