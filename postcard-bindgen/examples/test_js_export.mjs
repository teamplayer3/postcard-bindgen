import { serialize, deserialize } from "./js_export.js";

const test = {
    name: 23,
    other: 24322
}

const otherTest = {
    name: 2,
    other: 23232,
    test: test
}

const bytes = serialize("OtherTest", otherTest)
console.log(bytes)

const deser = deserialize("OtherTest", bytes)
console.log(deser)