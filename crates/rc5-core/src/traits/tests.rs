use core::ops::{Add, Div, Sub};

use typenum::{Max, Unsigned};

use super::*;

#[derive(Default, Clone, Copy)]
struct W1Byte(u8);
impl WordSize for W1Byte {
    type ByteLen = typenum::U1;
}

#[derive(Default, Clone, Copy)]
struct W2Byte(u16);
impl WordSize for W2Byte {
    type ByteLen = typenum::U2;
}

#[derive(Default, Clone, Copy)]
struct W4Byte(u32);
impl WordSize for W4Byte {
    type ByteLen = typenum::U4;
}

#[derive(Default, Clone, Copy)]
struct W8Byte(u64);
impl WordSize for W8Byte {
    type ByteLen = typenum::U8;
}

#[derive(Default, Clone, Copy)]
struct W16Byte(u128);
impl WordSize for W16Byte {
    type ByteLen = typenum::U16;
}

fn assert_sizes<W, KS>(_key_as_bytes: &KeyBytes<KS>, _key_as_words: &KeyLTable<KS, W>)
where
    KS: Add<W::ByteLen>,
    Sum<KS, W::ByteLen>: Sub<typenum::U1>,
    Diff<Sum<KS, W::ByteLen>, typenum::U1>: Div<W::ByteLen>,
    Quot<Diff<Sum<KS, W::ByteLen>, typenum::U1>, W::ByteLen>: Max<typenum::U1>,

    KeyLTableSize<KS, W::ByteLen>: ArrayLength<W>,
    KS: ArrayLength<u8>,

    W: WordSize,
{
}

macro_rules! assert_sizes_key_and_l_table {
    ($test_name: ident, $Word: ty, $KeySizeInBytes: ty, $KeySizeInWords: ty) => {
        #[test]
        fn $test_name() {
            let key_as_bytes = &[0u8; <$KeySizeInBytes>::USIZE];
            let key_as_bytes = GenericArray::<u8, $KeySizeInBytes>::from_slice(key_as_bytes);
            let key_as_words = &[Default::default(); <$KeySizeInWords>::USIZE];
            let key_as_words = GenericArray::<$Word, $KeySizeInWords>::from_slice(key_as_words);
            assert_sizes::<$Word, $KeySizeInBytes>(key_as_bytes, key_as_words);
        }
    };
}

assert_sizes_key_and_l_table!(k0_u8_1_time, u8, typenum::U0, typenum::U1);
assert_sizes_key_and_l_table!(k1_u8_1_time, u8, typenum::U1, typenum::U1);
assert_sizes_key_and_l_table!(k2_u8_1_time, u8, typenum::U2, typenum::U2);
assert_sizes_key_and_l_table!(k4_u8_1_time, u8, typenum::U4, typenum::U4);
assert_sizes_key_and_l_table!(k8_u8_2_times, u8, typenum::U8, typenum::U8);
assert_sizes_key_and_l_table!(k16_u8_4_times, u8, typenum::U16, typenum::U16);
assert_sizes_key_and_l_table!(k32_u8_8_times, u8, typenum::U32, typenum::U32);

assert_sizes_key_and_l_table!(k0_u16_1_time, u16, typenum::U0, typenum::U1);
assert_sizes_key_and_l_table!(k1_u16_1_time, u16, typenum::U1, typenum::U1);
assert_sizes_key_and_l_table!(k2_u16_1_time, u16, typenum::U2, typenum::U1);
assert_sizes_key_and_l_table!(k4_u16_1_time, u16, typenum::U4, typenum::U2);
assert_sizes_key_and_l_table!(k8_u16_2_times, u16, typenum::U8, typenum::U4);
assert_sizes_key_and_l_table!(k16_u16_4_times, u16, typenum::U16, typenum::U8);
assert_sizes_key_and_l_table!(k32_u16_8_times, u16, typenum::U32, typenum::U16);

assert_sizes_key_and_l_table!(k0_u32_1_time, u32, typenum::U0, typenum::U1);
assert_sizes_key_and_l_table!(k1_u32_1_time, u32, typenum::U1, typenum::U1);
assert_sizes_key_and_l_table!(k2_u32_1_time, u32, typenum::U2, typenum::U1);
assert_sizes_key_and_l_table!(k4_u32_1_time, u32, typenum::U4, typenum::U1);
assert_sizes_key_and_l_table!(k8_u32_2_times, u32, typenum::U8, typenum::U2);
assert_sizes_key_and_l_table!(k16_u32_4_times, u32, typenum::U16, typenum::U4);
assert_sizes_key_and_l_table!(k32_u32_8_times, u32, typenum::U32, typenum::U8);

assert_sizes_key_and_l_table!(k0_u64_1_time, u64, typenum::U0, typenum::U1);
assert_sizes_key_and_l_table!(k1_u64_1_time, u64, typenum::U1, typenum::U1);
assert_sizes_key_and_l_table!(k2_u64_1_time, u64, typenum::U2, typenum::U1);
assert_sizes_key_and_l_table!(k4_u64_1_time, u64, typenum::U4, typenum::U1);
assert_sizes_key_and_l_table!(k8_u64_2_times, u64, typenum::U8, typenum::U1);
assert_sizes_key_and_l_table!(k16_u64_4_times, u64, typenum::U16, typenum::U2);
assert_sizes_key_and_l_table!(k32_u64_8_times, u64, typenum::U32, typenum::U4);

assert_sizes_key_and_l_table!(k0_u128_1_time, u128, typenum::U0, typenum::U1);
assert_sizes_key_and_l_table!(k1_u128_1_time, u128, typenum::U1, typenum::U1);
assert_sizes_key_and_l_table!(k2_u128_1_time, u128, typenum::U2, typenum::U1);
assert_sizes_key_and_l_table!(k4_u128_1_time, u128, typenum::U4, typenum::U1);
assert_sizes_key_and_l_table!(k8_u128_2_times, u128, typenum::U8, typenum::U1);
assert_sizes_key_and_l_table!(k16_u128_4_times, u128, typenum::U16, typenum::U1);
assert_sizes_key_and_l_table!(k32_u128_8_times, u128, typenum::U32, typenum::U2);

assert_sizes_key_and_l_table!(k16_w1_16_times, W1Byte, typenum::U16, typenum::U16);
assert_sizes_key_and_l_table!(k16_w2_8_times, W2Byte, typenum::U16, typenum::U8);
assert_sizes_key_and_l_table!(k16_w4_4_times, W4Byte, typenum::U16, typenum::U4);
assert_sizes_key_and_l_table!(k16_w8_2_times, W8Byte, typenum::U16, typenum::U2);
assert_sizes_key_and_l_table!(k16_w16_1_time, W16Byte, typenum::U16, typenum::U1);
