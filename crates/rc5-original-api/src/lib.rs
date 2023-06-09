//! This crate provides the API originally proposed in the test-task.
//! 

use rc5_core::traits::{KeyBytes, KeyLTable, KeySTable, WordByteRepr, WordBytes};

type Word = u32;
type Arith = rc5_core::words::StdArith;
type ByteRepr = rc5_core::words::LittleEndian;
type Magic = rc5_core::words::StdMagic;

type R = typenum::U12;
type B = typenum::U16;

/*
 * This function should return a cipher text for a given key and plaintext
 *
 */
pub fn encode(key: Vec<u8>, plaintext: Vec<u8>) -> Vec<u8> {
    let key = KeyBytes::<B>::from_slice(&key[..]).to_owned();

    let mut l_table: KeyLTable<B, Word> = Default::default();
    let mut s_table: KeySTable<R, Word> = Default::default();
    rc5_core::algs::expand_key::<Word, R, B, Magic, ByteRepr, Arith>(
        &key,
        &mut l_table,
        &mut s_table,
    );

    let mut reg_a: Word = ByteRepr::from_bytes(WordBytes::<Word>::from_slice(&plaintext[0..4]));
    let mut reg_b: Word = ByteRepr::from_bytes(WordBytes::<Word>::from_slice(&plaintext[4..]));

    rc5_core::algs::encrypt::<Word, R, Arith>(&s_table, &mut reg_a, &mut reg_b);

    let mut ciphertext = vec![u8::default(); 8];

    ciphertext[0..4].copy_from_slice(ByteRepr::to_bytes(&reg_a).as_slice());
    ciphertext[4..].copy_from_slice(ByteRepr::to_bytes(&reg_b).as_slice());

    ciphertext
}

/*
 * This function should return a plaintext for a given key and ciphertext
 *
 */
pub fn decode(key: Vec<u8>, ciphertext: Vec<u8>) -> Vec<u8> {
    let key = KeyBytes::<B>::from_slice(&key[..]).to_owned();

    let mut l_table: KeyLTable<B, Word> = Default::default();
    let mut s_table: KeySTable<R, Word> = Default::default();
    rc5_core::algs::expand_key::<Word, R, B, Magic, ByteRepr, Arith>(
        &key,
        &mut l_table,
        &mut s_table,
    );

    let mut reg_a: Word = ByteRepr::from_bytes(WordBytes::<Word>::from_slice(&ciphertext[0..4]));
    let mut reg_b: Word = ByteRepr::from_bytes(WordBytes::<Word>::from_slice(&ciphertext[4..]));

    rc5_core::algs::decrypt::<Word, R, Arith>(&s_table, &mut reg_a, &mut reg_b);

    let mut plaintext = vec![u8::default(); 8];

    plaintext[0..4].copy_from_slice(ByteRepr::to_bytes(&reg_a).as_slice());
    plaintext[4..].copy_from_slice(ByteRepr::to_bytes(&reg_b).as_slice());

    plaintext
}
