use genco::quote;

use super::Tokens;

pub fn gen_util() -> Tokens {
    quote! {
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

        def check_bounds(n_bytes, signed, value, zero_able):
            if not zero_able:
                assert value != 0, "Value must not be zero"

            max = 2 ** (n_bytes * BITS_PER_BYTE)
            if signed:
                bounds = max / 2
                assert -bounds <= value < bounds, "Value is out of bounds ({}..{})".format(-bounds, bounds)
            else:
                assert 0 <= value < max, "Value is out of bounds (0..{})".format(max)
    }
}
