import { serialize, deserialize } from "../../target/debug/examples/js_export.js";

const otherTest = {
    name: 2,
    array: [12, 12, 2],
    allocArray: [1, 43, 1]
}

const bytes = serialize("OtherTest", otherTest)
console.log(bytes)

const deser = deserialize("OtherTest", bytes)
console.log(deser)