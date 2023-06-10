use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

use generic_array::ArrayLength;
use rc5_core::std_words::{LittleEndian, StdArith, StdMagic};
use rc5_core::traits::{KeyLTableSize, KeySTable, KeySTableSize, WordSize};
use typenum::{Diff, Max, Quot, Sum};

mod cipher_traits;

pub struct RC5<Word, R, B, ByteRepr = LittleEndian, Arith = StdArith, Magic = StdMagic>
where
    KeyLTableSize<B, Word::ByteLen>: ArrayLength<Word>,
    B: ArrayLength<u8>,
    Word: WordSize,

    B: Add<Word::ByteLen>,
    Sum<B, Word::ByteLen>: Sub<typenum::U1>,
    Diff<Sum<B, Word::ByteLen>, typenum::U1>: Div<Word::ByteLen>,
    Quot<Diff<Sum<B, Word::ByteLen>, typenum::U1>, Word::ByteLen>: Max<typenum::U1>,

    KeySTableSize<R>: ArrayLength<Word>,
    Sum<R, typenum::U1>: Mul<typenum::U2>,
    R: Add<typenum::U1>,
{
    s_table: KeySTable<R, Word>,
    _pd: PhantomData<(B, ByteRepr, Arith, Magic)>,
}
