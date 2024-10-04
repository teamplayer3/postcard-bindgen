

use genco::{prelude::python::Tokens, quote};

pub fn gen_ser_des_classes() -> Tokens {

    quote! {
        import struct
        
        BITS_PER_BYTE = 8
        BITS_PER_VARINT_BYTE = 7
        U8_BYTES = 1
        U16_BYTES = 2
        U32_BYTES = 4
        U64_BYTES = 8
        U128_BYTES = 16

        def de_zig_zag_signed(n):
            return (n >> 1) ^ (-(n & 0b1))

        def zig_zag(n_bytes, n):
            return (n << 1) ^ (n >> (n_bytes * BITS_PER_BYTE - 1))

        def varint_max(n_bytes):
            return (n_bytes * BITS_PER_BYTE + (BITS_PER_BYTE - 1)) // BITS_PER_VARINT_BYTE

        def max_of_last_byte(n_bytes):
            return (1 << (n_bytes * BITS_PER_BYTE) % 7) - 1

        def to_number_if_safe(n):
            return n if abs(n) > (1 << 53) - 1 else int(n)

        def varint(n_bytes, n):
            value = n
            out = []
            for i in range(varint_max(n_bytes)):
                out.append(int(value & 0xFF))
                if value < 128:
                    return out
                out[i] |= 0x80
                value >>= 7
            return out

        class Deserializer:
            def __init__(self, bytes_in):
                self.bytes = list(bytes_in)

            def pop_next(self):
                if not self.bytes:
                    raise Exception("input buffer too small")
                return self.bytes.pop(0)

            def pop_n(self, n):
                bytes_out = []
                for _ in range(n):
                    bytes_out.append(self.pop_next())
                return bytes_out

            def get_uint8(self):
                return self.pop_next()

            def try_take(self, n_bytes):
                out = 0
                v_max = varint_max(n_bytes)
                for i in range(v_max):
                    val = self.pop_next()
                    carry = val & 0x7F
                    out |= carry << (7 * i)
                    if (val & 0x80) == 0:
                        if i == v_max - 1 and val > max_of_last_byte(n_bytes):
                            raise Exception("Bad Variant")
                        else:
                            return out
                raise Exception("Bad Variant")

            def deserialize_bool(self):
                byte = self.pop_next()
                return byte is not None and byte > 0

            def deserialize_number(self, n_bytes, signed):
                if n_bytes == U8_BYTES:
                    return self.get_uint8()
                elif n_bytes in {U16_BYTES, U32_BYTES, U64_BYTES, U128_BYTES}:
                    val = self.try_take(n_bytes)
                    return to_number_if_safe(de_zig_zag_signed(val) if signed else val)
                else:
                    raise Exception("byte count not supported")

            def deserialize_number_float(self, n_bytes):
                b_buffer = bytes(self.pop_n(n_bytes))
                if n_bytes == U32_BYTES:
                    return struct.unpack("<f", b_buffer)[0]
                elif n_bytes == U64_BYTES:
                    return struct.unpack("<d", b_buffer)[0]
                else:
                    raise Exception("byte count not supported")

            def deserialize_string(self):
                str_len = self.try_take(U32_BYTES)
                str_bytes = self.pop_n(str_len)
                return "".join(chr(b) for b in str_bytes)

            def deserialize_array(self, des):
                return [des(self) for _ in range(self.try_take(U32_BYTES))]

            def deserialize_string_key_map(self, des):
                size = self.try_take(U32_BYTES)
                return {self.deserialize_string(): des(self) for _ in range(size)}

            def deserialize_map(self, des):
                size = self.try_take(U32_BYTES)
                return {des(self)[0]: des(self)[1] for _ in range(size)}

        class Serializer:
            def __init__(self):
                self.bytes = []

            def finish(self):
                return self.bytes

            def push_n(self, bytes_in):
                self.bytes.extend(bytes_in)

            def serialize_bool(self, value):
                self.serialize_number(U8_BYTES, False, 1 if value else 0)

            def serialize_number(self, n_bytes, signed, value):
                if n_bytes == U8_BYTES:
                    self.bytes.append(value)
                elif n_bytes in {U16_BYTES, U32_BYTES, U64_BYTES, U128_BYTES}:
                    value_b = int(value)
                    buffer = varint(n_bytes, zig_zag(n_bytes, value_b) if signed else value_b)
                    self.push_n(buffer)
                else:
                    raise Exception("byte count not supported")

            def serialize_number_float(self, n_bytes, value):
                if n_bytes == U32_BYTES:
                    b_buffer = struct.pack("<f", value)
                elif n_bytes == U64_BYTES:
                    b_buffer = struct.pack("<d", value)
                else:
                    raise Exception("byte count not supported")
                self.push_n(b_buffer)

            def serialize_string(self, s):
                self.push_n(varint(U32_BYTES, len(s)))
                self.push_n([ord(c) for c in s])

            def serialize_array(self, ser, array):
                self.push_n(varint(U32_BYTES, len(array)))
                for v in array:
                    ser(self, v)

            def serialize_string_key_map(self, ser, obj):
                entries = list(obj.items())
                self.push_n(varint(U32_BYTES, len(entries)))
                for k, v in entries:
                    self.serialize_string(k)
                    ser(self, v)

            def serialize_map(self, ser, map_obj):
                entries = list(map_obj.items())
                self.push_n(varint(U32_BYTES, len(entries)))
                for k, v in entries:
                    ser(self, k, v)

        def check_bounds(n_bytes, signed, value):
            max_val = 2 ** (n_bytes * BITS_PER_BYTE)
            value_b = int(value)
            if signed:
                bounds = max_val // 2
                return -bounds <= value_b < bounds
            else:
                return 0 <= value_b < max_val
    }
}