from py_test_bindings import *
from json import dumps

all_tests = AllTests(
    a=ContainerTypes(
        u=UnionContainer(),
        e_a=EnumContainer_A(),
        e_b=EnumContainer_B(123),
        e_c=EnumContainer_C(123, StructContainer(a=123, b=123, c=123, d=123)),
        e_d=EnumContainer_D(a=123, b=StructContainer(
            a=123, b=123, c=123, d=123)),
        t=TupleContainer(123, StructContainer(
            a=123, b=123, c=123, d=123), EnumContainer_A()),
        s=StructContainer(a=123, b=123, c=123, d=123)
    ),
    b=PrimitiveTypes(
        u8=255,
        u16=65535,
        u32=4294967295,
        u64=18446744073709551615,
        u128=340282366920938463463374607431768211455,
        usize=18446744073709551615,
        i8_max=127,
        i8_min=-128,
        i16_max=32767,
        i16_min=-32768,
        i32_max=2147483647,
        i32_min=-2147483648,
        i64_max=9223372036854775807,
        i64_min=-9223372036854775808,
        i128_max=170141183460469231731687303715884105727,
        i128_min=-170141183460469231731687303715884105728,
        isize_max=9223372036854775807,
        isize_min=-9223372036854775808,
        f32=123.123,
        f64=123.123,
        bool_true=True,
        bool_false=False,
        none_zero=123
    ),
    c=CompoundTypes(
        static_byte_slice=[123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        static_str="Hello",
        array=[123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        range=range(10, 20),
        option_some=123,
        option_none=None,
        tuple=(
            123,
            StructContainer(a=123, b=123, c=123, d=123),
            EnumContainer_A(),
            TupleContainer(123, StructContainer(
                a=123, b=123, c=123, d=123), EnumContainer_A())
        )
    ),
    d=AllocTypes(
        a=[123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        b="Hello",
        c={123: 123}
    ),
    e=HeaplessTypes(
        a=[123, 123, 123, 123, 123, 123, 123, 123, 123, 123],
        b="Hello",
        c={123: 123}
    ),
    f=e.E(123, e.f.F(123))
)

ser = open("serialized.bytes", "rb").read()
d, _bytes = deserialize(AllTests, ser)
print(d)

ser_own = serialize(all_tests)
print(ser_own)

d_des, _bytes = deserialize(AllTests, ser_own)
print(d_des)

assert ser == ser_own
