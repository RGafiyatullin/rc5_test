use crate::traits::{KeyBytes, KeyLTable, KeySTable, WordByteRepr, WordBytes, WordSize};
use std::string::String;
use typenum::Unsigned;

type Word = u32;
type Arith = crate::std_words::StdArith;
type ByteRepr = crate::std_words::LittleEndian;
type Magic = crate::std_words::StdMagic;

type R = typenum::U12;
type B = typenum::U16;

fn parse_key(key_hex: &str) -> KeyBytes<B> {
    let mut key: KeyBytes<B> = Default::default();
    hex::decode_to_slice(key_hex, key.as_mut_slice()).expect("key len?");
    key
}

fn parse_text(text_hex: &str) -> (Word, Word) {
    let mut text = [0u8; <Word as WordSize>::ByteLen::USIZE * 2];
    hex::decode_to_slice(text_hex, &mut text[..]).expect("text len?");
    let reg_a = ByteRepr::from_bytes(WordBytes::<Word>::from_slice(&text[0..4]));
    let reg_b = ByteRepr::from_bytes(WordBytes::<Word>::from_slice(&text[4..]));

    (reg_a, reg_b)
}

fn render_text(reg_a: Word, reg_b: Word) -> String {
    let mut text = [0u8; <Word as WordSize>::ByteLen::USIZE * 2];
    text[0..4].copy_from_slice(ByteRepr::to_bytes(&reg_a).as_slice());
    text[4..].copy_from_slice(ByteRepr::to_bytes(&reg_b).as_slice());
    hex::encode(&text)
}

#[test]
fn round_trip() {
    let key = parse_key("000102030405060708090a0b0c0d0e0f");

    let mut l_table: KeyLTable<B, Word> = Default::default();
    let mut s_table: KeySTable<R, Word> = Default::default();
    crate::algs::expand_key::<Word, R, B, Magic, ByteRepr, Arith>(&key, &mut l_table, &mut s_table);

    let (mut reg_a, mut reg_b) = parse_text("0001020304050607");
    crate::algs::encrypt::<Word, R, Arith>(&s_table, &mut reg_a, &mut reg_b);
    crate::algs::decrypt::<Word, R, Arith>(&s_table, &mut reg_a, &mut reg_b);

    assert_eq!(render_text(reg_a, reg_b), "0001020304050607");
}

fn run_case(key: &str, plaintext_hex: &str, ciphertext_hex: &str) {
    let key = parse_key(key);
    let (mut reg_a, mut reg_b) = parse_text(plaintext_hex);

    let mut l_table: KeyLTable<B, Word> = Default::default();
    let mut s_table: KeySTable<R, Word> = Default::default();
    crate::algs::expand_key::<Word, R, B, Magic, ByteRepr, Arith>(&key, &mut l_table, &mut s_table);

    crate::algs::encrypt::<Word, R, Arith>(&s_table, &mut reg_a, &mut reg_b);
    assert_eq!(render_text(reg_a, reg_b), ciphertext_hex);

    crate::algs::decrypt::<Word, R, Arith>(&s_table, &mut reg_a, &mut reg_b);
    assert_eq!(render_text(reg_a, reg_b), plaintext_hex);
}

#[test]
fn ex_1() {
    run_case(
        "00000000000000000000000000000000",
        "0000000000000000",
        "21a5dbee154b8f6d",
    );
}

#[test]
fn ex_2() {
    run_case(
        "915f4619be41b2516355a50110a9ce91",
        "21a5dbee154b8f6d",
        "f7c013ac5b2b8952",
    );
}

#[test]
fn ex_3() {
    run_case(
        "783348e75aeb0f2fd7b169bb8dc16787",
        "f7c013ac5b2b8952",
        "2f42b3b70369fc92",
    );
}

#[test]
fn ex_4() {
    run_case(
        "dc49db1375a5584f6485b413b5f12baf",
        "2f42b3b70369fc92",
        "65c178b284d197cc",
    );
}

#[test]
fn ex_5() {
    run_case(
        "5269f149d41ba0152497574d7f153125",
        "65c178b284d197cc",
        "eb44e415da319824",
    );
}
