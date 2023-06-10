use typenum::*;

use crate::rc5::RC5;

use rc5_core::strange_words::{
    StrangeArithmetics, StrangeByteRepr, StrangeMagic, W24bit, W32bit, W80bit,
};

use super::common::run_case;

#[test]
fn rc5_8_12_4() {
    run_case::<RC5<u8, U12, U4>>("00010203", "0001", "212A")
}

#[test]
#[ignore]
fn rc5_16_16_8() {
    run_case::<RC5<u16, U16, U8>>("0001020304050607", "00010203", "23A8D72E")
}

#[test]
fn rc5_32_20_16() {
    run_case::<RC5<u32, U20, U16>>(
        "000102030405060708090A0B0C0D0E0F",
        "0001020304050607",
        "2A0EDC0E9431FF73",
    )
}

#[test]
fn rc5_64_24_24() {
    run_case::<RC5<u64, U24, U24>>(
        "000102030405060708090A0B0C0D0E0F1011121314151617",
        "000102030405060708090A0B0C0D0E0F",
        "A46772820EDBCE0235ABEA32AE7178DA",
    )
}

#[test]
fn rc5_128_28_32() {
    run_case::<RC5<u128, U28, U32>>(
        "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
        "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
        "ECA5910921A4F4CFDD7AD7AD20A1FCBA068EC7A7CD752D68FE914B7FE180B440",
    )
}

#[test]
fn rc5_32_20_16_with_strange_word() {
    run_case::<RC5<W32bit, U20, U16, StrangeByteRepr, StrangeArithmetics, StrangeMagic>>(
        "000102030405060708090A0B0C0D0E0F",
        "0001020304050607",
        "2A0EDC0E9431FF73",
    )
}

#[test]
#[ignore]
fn rc5_24_4_0() {
    run_case::<RC5<W24bit, U4, U0, StrangeByteRepr, StrangeArithmetics, StrangeMagic>>(
        "",
        "000102030405",
        "89CBDCC9525A",
    )
}

#[test]
#[ignore]
fn rc5_80_4_12() {
    run_case::<RC5<W80bit, U4, U12, StrangeByteRepr, StrangeArithmetics, StrangeMagic>>(
        "000102030405060708090A0B",
        "000102030405060708090A0B0C0D0E0F10111213",
        "9CB59ECBA4EA84568A4278B0E132D5FC9D5819D6",
    )
}
