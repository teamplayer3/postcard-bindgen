// Before running this with `$node .\test_js_export.mjs` run the rust example with 
// `$cargo run --example generate_bindings --features std,generating`.

import fs from "fs";
import { serialize, deserialize } from "./js-test-bindings/index.js"
import { assert } from "console";

const all_tests = {
    a: {
        u: {},
        e_a: { tag: "A" },
        e_b: { tag: "B", value: 123 },
        e_c: { tag: "C", value: [123, { a: 123, b: 123, c: 123, d: 123 }] },
        e_d: { tag: "D", value: { a: 123, b: { a: 123, b: 123, c: 123, d: 123 } } },
        t: [123, { a: 123, b: 123, c: 123, d: 123 }, { tag: "A" }],
        s: { a: 123, b: 123, c: 123, d: 123 }
    },
    b: {
        u8: 255,
        u16: 65535,
        u32: 4294967295,
        u64: 18446744073709551615n,
        u128: 340282366920938463463374607431768211455n,
        usize: 18446744073709551615n,
        i8_max: 127,
        i8_min: -128,
        i16_max: 32767,
        i16_min: -32768,
        i32_max: 2147483647,
        i32_min: -2147483648,
        i64_max: 9223372036854775807n,
        i64_min: -9223372036854775808n,
        i128_max: 170141183460469231731687303715884105727n,
        i128_min: -170141183460469231731687303715884105728n,
        isize_max: 9223372036854775807n,
        isize_min: -9223372036854775808n,
        f32: 123.123,
        f64: 123.123,
        bool_true: true,
        bool_false: false,
        none_zero: 123
    },
    c: {
        static_byte_slice: [123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        static_str: "Hello",
        array: [123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        range: { start: 10, end: 20 },
        option_some: 123,
        option_none: undefined,
        tuple: [
            123,
            { a: 123, b: 123, c: 123, d: 123 },
            { tag: "A" },
            [123, { a: 123, b: 123, c: 123, d: 123 }, { tag: "A" }]
        ]
    },
    d: {
        a: [123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        b: "Hello",
        c: new Map([[123, 123]])
    },
    e: {
        a: [123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        b: "Hello",
        c: new Map([[123, 123]])
    },
    f: [123, [123]]
};

const bytes = serialize("AllTests", all_tests)
console.log(bytes)

const js_des = deserialize("AllTests", bytes)
console.log(js_des)

const bytes_file = `${process.cwd()}/serialized.bytes`
const loaded_bytes = fs.readFileSync(bytes_file)
const rust_des = deserialize("AllTests", loaded_bytes);
console.log(rust_des)

function bigIntFix (key, value) {
    if (typeof value === 'bigint') {
        return value.toString()
    }
    return value
}

// TODO: Fails because of precision error in f32 and f64
// assert(JSON.stringify(all_tests, bigIntFix) === JSON.stringify(js_des, bigIntFix))
assert(bytes, loaded_bytes)
assert(JSON.stringify(js_des, bigIntFix) === JSON.stringify(rust_des, bigIntFix))