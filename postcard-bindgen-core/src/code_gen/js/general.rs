use genco::quote;

use super::Tokens;

pub fn gen_util() -> Tokens {
    quote! {
        const BITS_PER_BYTE = 8, BITS_PER_VARINT_BYTE = 7, U8_BYTES = 1, U16_BYTES = 2, U32_BYTES = 4, U64_BYTES = 8, U128_BYTES = 16

        const de_zig_zag_signed = (n) => (n >> 1n) ^ (-(n & 0b1n))
        const zig_zag = (n_bytes, n) => (n << 1n) ^ (n >> BigInt(n_bytes * BITS_PER_BYTE - 1))
        const varint_max = (n_bytes) => Math.floor((n_bytes * BITS_PER_BYTE + (BITS_PER_BYTE - 1)) / BITS_PER_VARINT_BYTE)
        const max_of_last_byte = (n_bytes) => (1 << (n_bytes * BITS_PER_BYTE) % 7) - 1
        const to_number_if_safe = (n) => Number.MAX_SAFE_INTEGER < ((n < 0n) ? -n : n) ? n : Number(n)
        const varint = (n_bytes, n) => { let value = BigInt(n), out = []; for (let i = 0; i < varint_max(n_bytes); i++) { out.push(Number(value & 0xFFn)); if (value < 128n) { return out } out[i] |= 0x80; value >>= 7n } }
    }
}
