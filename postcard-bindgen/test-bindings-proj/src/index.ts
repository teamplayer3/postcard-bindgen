// import { readFileSync } from "fs";
// import { deserialize, serialize } from "js-test-bindings";

import { serialize } from "js-test-bindings"

console.log("neeee")
const bytes = serialize("a.FooBar", { a: 123, b: "hello" })
console.log(bytes)
// const b = deserialize("a.FooBar", bytes)
// console.log(b)

// const bytes_file = `${process.cwd()}/../serialized.bytes`
// const loaded_bytes = readFileSync(bytes_file)
// const rust_des = deserialize("Message", [...loaded_bytes]);
// console.log(rust_des)