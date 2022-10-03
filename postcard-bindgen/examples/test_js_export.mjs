import { serialize, deserialize } from "./js_export.js";

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