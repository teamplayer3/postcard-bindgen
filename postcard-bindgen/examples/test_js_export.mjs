import { serialize, deserialize } from "./js_export.js";

const test = {
    name: 23,
    other: 24322
}

const otherTest = {
    name: 2,
    test: test
}

const tupleStruct = [234, 231, 4523]

const bytes = serialize("TupleStruct", tupleStruct)
console.log(bytes)

const deser = deserialize("TupleStruct", bytes)
console.log(deser)