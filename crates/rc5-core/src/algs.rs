use core::ops::{Add, Div, Mul, Sub};

use generic_array::ArrayLength;
use typenum::{Diff, Max, Quot, Sum, Unsigned};

use crate::traits::{
    Arithmetics, KeyBytes, KeyLTable, KeyLTableSize, KeySTable, KeySTableSize, Magic, WordByteRepr,
    WordSize,
};

mod key_table;

pub fn expand_key<Word, R, B, M, WBR, A>(
    key_bytes: &KeyBytes<B>,
    l_table: &mut KeyLTable<B, Word>,
    s_table: &mut KeySTable<R, Word>,
) where
    KeyLTableSize<B, Word::ByteLen>: ArrayLength<Word>,
    B: ArrayLength<u8>,
    Word: WordSize + Default + Copy,

    B: Add<Word::ByteLen>,
    Sum<B, Word::ByteLen>: Sub<typenum::U1>,
    Diff<Sum<B, Word::ByteLen>, typenum::U1>: Div<Word::ByteLen>,
    Quot<Diff<Sum<B, Word::ByteLen>, typenum::U1>, Word::ByteLen>: Max<typenum::U1>,

    KeySTableSize<R>: ArrayLength<Word>,
    Sum<R, typenum::U1>: Mul<typenum::U2>,
    R: Add<typenum::U1>,

    WBR: WordByteRepr<Word>,
    A: Arithmetics<Word>,
    M: Magic<Word>,
{
    key_table::l_table_init::<Word, B, WBR>(key_bytes, l_table);
    key_table::s_table_init::<Word, R, M, A>(s_table);
    key_table::s_table_mix_secret_key::<Word, B, R, A, M>(l_table, s_table);
}

pub fn encrypt<Word, R, A>(s_table: &KeySTable<R, Word>, reg_a: &mut Word, reg_b: &mut Word)
where
    Sum<R, typenum::U1>: Mul<typenum::U2>,
    R: Add<typenum::U1> + Unsigned,
    KeySTableSize<R>: ArrayLength<Word>,
    A: Arithmetics<Word>,
{
    *reg_a = A::add(reg_a, &s_table[0]);
    *reg_b = A::add(reg_b, &s_table[1]);

    for i in 1..=R::USIZE {
        *reg_a = A::xor(reg_a, reg_b);
        *reg_a = A::rotl(reg_a, reg_b);
        *reg_a = A::add(reg_a, &s_table[2 * i]);

        *reg_b = A::xor(reg_b, reg_a);
        *reg_b = A::rotl(reg_b, reg_a);
        *reg_b = A::add(reg_b, &s_table[2 * i + 1]);
    }
}

pub fn decrypt<Word, R, A>(s_table: &KeySTable<R, Word>, reg_a: &mut Word, reg_b: &mut Word)
where
    Sum<R, typenum::U1>: Mul<typenum::U2>,
    R: Add<typenum::U1> + Unsigned,
    KeySTableSize<R>: ArrayLength<Word>,
    A: Arithmetics<Word>,
{
    for i in (1..=R::USIZE).rev() {
        *reg_b = A::sub(reg_b, &s_table[2 * i + 1]);
        *reg_b = A::rotr(reg_b, reg_a);
        *reg_b = A::xor(reg_b, reg_a);

        *reg_a = A::sub(reg_a, &s_table[2 * i]);
        *reg_a = A::rotr(reg_a, reg_b);
        *reg_a = A::xor(reg_a, reg_b);
    }

    *reg_b = A::sub(reg_b, &s_table[1]);
    *reg_a = A::sub(reg_a, &s_table[0]);
}
