import { serialize, deserialize } from "./js_export.js";

const otherTest = {
    name: 2,
    other: 12,
    array: [12, 12, 2],
    alloc_array: [1, 43, 1]
}

const bytes = serialize("OtherTest", otherTest)
console.log(bytes)

const deser = deserialize("OtherTest", bytes)
console.log(deser)