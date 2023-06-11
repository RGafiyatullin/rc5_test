use core::ops::{Add, Div, Mul, Sub};

use cipher::{
    BlockBackend, BlockCipher, BlockDecrypt, BlockEncrypt, BlockSizeUser, KeyInit, KeySizeUser,
    ParBlocksSizeUser,
};
use generic_array::ArrayLength;
use typenum::{Diff, Max, Prod, Quot, Sum, Unsigned};

use rc5_core::traits::{
    Arithmetics, BlockSize, KeyLTable, KeyLTableSize, KeySTable, KeySTableSize, Magic,
    WordByteRepr, WordBytes, WordSize,
};

use super::RC5;

impl<Word, R, B, BR, A, M> BlockSizeUser for RC5<Word, R, B, BR, A, M>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,
{
    type BlockSize = BlockSize<Word>;
}

impl<Word, R, B, BR, A, M> BlockCipher for RC5<Word, R, B, BR, A, M>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,
{
}

impl<Word, R, B, BR, A, M> KeySizeUser for RC5<Word, R, B, BR, A, M>
where
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
{
    type KeySize = B;
}

impl<Word, R, B, BR, A, M> KeyInit for RC5<Word, R, B, BR, A, M>
where
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

    BR: WordByteRepr<Word>,
    A: Arithmetics<Word>,
    M: Magic<Word>,

    Word: core::fmt::Debug,
{
    fn new(key: &cipher::Key<Self>) -> Self {
        let mut l_table = KeyLTable::<B, Word>::default();
        let mut s_table = KeySTable::<R, Word>::default();

        rc5_core::algs::expand_key::<Word, R, B, M, BR, A>(key, &mut l_table, &mut s_table);

        // TODO: zeroize l_table?

        Self {
            s_table,
            _pd: Default::default(),
        }
    }
}

impl<Word, R, B, BR, A, M> BlockEncrypt for RC5<Word, R, B, BR, A, M>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,

    R: Unsigned,
    BR: WordByteRepr<Word>,
    A: Arithmetics<Word>,
{
    fn encrypt_with_backend(&self, f: impl cipher::BlockClosure<BlockSize = Self::BlockSize>) {
        f.call(&mut Backend(self, rc5_core::algs::encrypt::<Word, R, A>))
    }
}

impl<Word, R, B, BR, A, M> BlockDecrypt for RC5<Word, R, B, BR, A, M>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,

    R: Unsigned,
    BR: WordByteRepr<Word>,
    A: Arithmetics<Word>,
{
    fn decrypt_with_backend(&self, f: impl cipher::BlockClosure<BlockSize = Self::BlockSize>) {
        f.call(&mut Backend(self, rc5_core::algs::decrypt::<Word, R, A>))
    }
}

struct Backend<T, F>(T, F);

impl<'a, Word, R, B, BR, A, M, F> BlockSizeUser for Backend<&'a RC5<Word, R, B, BR, A, M>, F>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,
{
    type BlockSize = <RC5<Word, R, B> as BlockSizeUser>::BlockSize;
}

impl<'a, Word, R, B, BR, A, M, F> ParBlocksSizeUser for Backend<&'a RC5<Word, R, B, BR, A, M>, F>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,
{
    type ParBlocksSize = typenum::U1;
}

impl<'a, Word, R, B, BR, A, M, F> BlockBackend for Backend<&'a RC5<Word, R, B, BR, A, M>, F>
where
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

    Word::ByteLen: Mul<typenum::U2>,
    Prod<Word::ByteLen, typenum::U2>: ArrayLength<u8>,

    BR: WordByteRepr<Word>,

    F: Fn(&KeySTable<R, Word>, &mut Word, &mut Word),
{
    fn proc_block(&mut self, mut io: cipher::inout::InOut<'_, '_, cipher::Block<Self>>) {
        let block = io.get_in();
        let mut reg_a = BR::from_bytes(WordBytes::<Word>::from_slice(
            &block[..Word::ByteLen::USIZE],
        ));
        let mut reg_b = BR::from_bytes(WordBytes::<Word>::from_slice(
            &block[Word::ByteLen::USIZE..],
        ));

        (self.1)(&self.0.s_table, &mut reg_a, &mut reg_b);

        let block = io.get_out();
        block[..Word::ByteLen::USIZE].copy_from_slice(BR::to_bytes(&reg_a).as_ref());
        block[Word::ByteLen::USIZE..].copy_from_slice(BR::to_bytes(&reg_b).as_ref());
    }
}
