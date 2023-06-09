use generic_array::{ArrayLength, GenericArray};
use typenum::{Diff, Maximum, Prod, Quot, Sum};

pub type WordBytes<Word> = GenericArray<u8, <Word as WordSize>::ByteLen>;

pub type BlockSize<Word> = Prod<<Word as WordSize>::ByteLen, typenum::U2>;
pub type BlockBytes<Word> = GenericArray<u8, BlockSize<Word>>;

pub type KeyBytes<B> = GenericArray<u8, B>;

pub type KeyLTableSize<B, W> = Maximum<Quot<Diff<Sum<B, W>, typenum::U1>, W>, typenum::U1>;
pub type KeyLTable<B, Word> = GenericArray<Word, KeyLTableSize<B, <Word as WordSize>::ByteLen>>;

pub type KeySTableSize<R> = Prod<Sum<R, typenum::U1>, typenum::U2>;
pub type KeySTable<R, Word> = GenericArray<Word, KeySTableSize<R>>;

pub trait Arithmetics<W> {
    fn add(l: &W, r: &W) -> W;
    fn sub(l: &W, r: &W) -> W;

    fn rotl(l: &W, r: &W) -> W;
    fn rotr(l: &W, r: &W) -> W;

    fn xor(l: &W, r: &W) -> W;
}

pub trait Magic<W> {
    const P: W;
    const Q: W;

    const THREE: W;
}

pub trait WordSize {
    type ByteLen: ArrayLength<u8>;
}

pub trait WordByteRepr<W>
where
    W: WordSize,
{
    fn to_bytes(w: &W) -> WordBytes<W>;
    fn from_bytes(b: &WordBytes<W>) -> W;
}

#[cfg(test)]
mod tests;
