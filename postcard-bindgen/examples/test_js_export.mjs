import { serialize, deserialize } from "./js_export.js";

const test = {
    name: 23,
    other: 24322
}

const otherTest = {
    name: 2,
    test: test,
    tuple: [123, 124, 1],
    unit: {},
    enum_ty: {
        key: "B",
        inner: [2345]
    }
}

const bytes = serialize("OtherTest", otherTest)
console.log(bytes)

const deser = deserialize("OtherTest", bytes)
console.log(deser)

const enum_1 = "A"
const enum_2 = {
    type: "B",
    inner: [23, 2312]
}
const enum_3 = "C"
const enum_4 = {
    type: "D",
    inner: {
        a: 123,
        b: 12343
    }
}

// enum {
//     A, B(u8, u16), C, D { a: u8, b: u64 }
// }