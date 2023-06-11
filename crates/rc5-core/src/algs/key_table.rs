use core::ops::{Add, Div, Mul, Sub};

use generic_array::ArrayLength;
use typenum::{Diff, Max, Quot, Sum, Unsigned};

use crate::traits::{
    Arithmetics, KeyBytes, KeyLTable, KeyLTableSize, KeySTable, KeySTableSize, Magic, WordByteRepr,
    WordBytes, WordSize,
};

pub fn l_table_init<Word, B, WBR>(key: &KeyBytes<B>, l_table: &mut KeyLTable<B, Word>)
where
    B: Add<Word::ByteLen>,
    Sum<B, Word::ByteLen>: Sub<typenum::U1>,
    Diff<Sum<B, Word::ByteLen>, typenum::U1>: Div<Word::ByteLen>,
    Quot<Diff<Sum<B, Word::ByteLen>, typenum::U1>, Word::ByteLen>: Max<typenum::U1>,
    KeyLTableSize<B, Word::ByteLen>: ArrayLength<Word>,

    B: ArrayLength<u8>,
    Word: WordSize,
    WBR: WordByteRepr<Word>,
{
    let key_bytes = key.as_slice();
    let key_words = l_table.as_mut_slice();

    for (dst_word, src_bytes) in key_words.iter_mut().zip(key_bytes.chunks(Word::ByteLen::USIZE)) {
        let mut w_bytes = WordBytes::<Word>::default();

        for (dst_byte, src_byte) in
            w_bytes.as_mut_slice().iter_mut().rev().zip(src_bytes.iter().rev())
        {
            *dst_byte = *src_byte;
        }

        *dst_word = WBR::from_bytes(&w_bytes);
    }
}

pub fn s_table_init<Word, R, M, A>(s_table: &mut KeySTable<R, Word>)
where
    A: Arithmetics<Word>,
    M: Magic<Word>,
    KeySTableSize<R>: ArrayLength<Word>,
    Sum<R, typenum::U1>: Mul<typenum::U2>,
    R: Add<typenum::U1>,
{
    s_table[0] = M::P;
    for i in 1..KeySTableSize::<R>::USIZE {
        s_table[i] = A::add(&s_table[i - 1], &M::Q);
    }
}

pub fn s_table_mix_secret_key<Word, B, R, A, M>(
    l_table: &mut KeyLTable<B, Word>,
    s_table: &mut KeySTable<R, Word>,
) where
    B: Add<Word::ByteLen>,
    Sum<B, Word::ByteLen>: Sub<typenum::U1>,
    Diff<Sum<B, Word::ByteLen>, typenum::U1>: Div<Word::ByteLen>,
    Quot<Diff<Sum<B, Word::ByteLen>, typenum::U1>, Word::ByteLen>: Max<typenum::U1>,
    KeyLTableSize<B, Word::ByteLen>: ArrayLength<Word>,

    Sum<R, typenum::U1>: Mul<typenum::U2>,
    R: Add<typenum::U1>,
    KeySTableSize<R>: ArrayLength<Word>,

    B: ArrayLength<u8>,
    Word: WordSize + Default + Copy,
    A: Arithmetics<Word>,
    M: Magic<Word>,
{
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut a: Word = Default::default();
    let mut b: Word = Default::default();

    let t = KeySTableSize::<R>::USIZE;
    let c = KeyLTableSize::<B, Word::ByteLen>::USIZE;
    for _ in 0..(3 * t.max(c)) {
        a = A::rotl(&A::add(&s_table[i], &A::add(&a, &b)), &M::THREE);
        s_table[i] = a;

        b = A::rotl(&A::add(&l_table[j], &A::add(&a, &b)), &A::add(&a, &b));
        l_table[j] = b;

        i = (i + 1) % t;
        j = (j + 1) % c;
    }
}

#[test]
fn test_key_bytes_to_words_01() {
    use crate::std_words::LittleEndian;

    type W = u32;
    type KS = typenum::U16;

    let key_bytes = &[0x01u8; 16];
    let key_bytes = KeyBytes::<KS>::from_slice(key_bytes);

    let mut key_words = KeyLTable::<KS, W>::default();

    l_table_init::<W, KS, LittleEndian>(key_bytes, &mut key_words);

    assert_eq!(key_words.as_slice(), &[0x01010101u32; 4]);
}

#[test]
fn test_s_table_init() {
    use crate::std_words::StdArith;
    use crate::std_words::StdMagic;

    type R = typenum::U12;
    type W = u32;

    let mut s_table = Default::default();
    s_table_init::<W, R, StdMagic, StdArith>(&mut s_table);

    let s_table_expected: [W; 2 * (R::USIZE + 1)] = {
        let p = <StdMagic as Magic<W>>::P;
        let q = <StdMagic as Magic<W>>::Q;

        core::array::from_fn(|i| p.wrapping_add((i as W).wrapping_mul(q)))
    };

    assert_eq!(&s_table[..], &s_table_expected[..])
}
