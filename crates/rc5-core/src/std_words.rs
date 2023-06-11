use crate::traits::Arithmetics;
use crate::traits::Magic;
use crate::traits::WordByteRepr;
use crate::traits::WordSize;

pub enum LittleEndian {}
pub enum StdMagic {}

pub enum StdArith {}

macro_rules! impl_word {
    ($W: ty, $S: ty, $p: literal, $q: literal) => {
        impl WordSize for $W {
            type ByteLen = $S;
        }

        impl WordByteRepr<$W> for LittleEndian {
            fn from_bytes(b: &crate::traits::WordBytes<$W>) -> $W {
                <$W>::from_le_bytes(*b.as_ref())
            }
            fn to_bytes(w: &$W) -> crate::traits::WordBytes<$W> {
                w.to_le_bytes().into()
            }
        }

        impl Magic<$W> for StdMagic {
            const THREE: $W = 3;

            const P: $W = $p;
            const Q: $W = $q;
        }

        impl Arithmetics<$W> for StdArith {
            #[inline(always)]
            fn add(l: &$W, r: &$W) -> $W {
                l.wrapping_add(*r)
            }

            #[inline(always)]
            fn sub(l: &$W, r: &$W) -> $W {
                l.wrapping_sub(*r)
            }

            #[inline(always)]
            fn rotl(l: &$W, r: &$W) -> $W {
                l.rotate_left(*r as u32)
            }

            #[inline(always)]
            fn rotr(l: &$W, r: &$W) -> $W {
                l.rotate_right(*r as u32)
            }

            #[inline(always)]
            fn xor(l: &$W, r: &$W) -> $W {
                core::ops::BitXor::bitxor(*l, *r)
            }
        }
    };
}

impl_word!(u8, typenum::U1, 0xB7, 0x9F);
impl_word!(u16, typenum::U2, 0xB7E1, 0x9E37);
impl_word!(u32, typenum::U4, 0xB7E1_5163, 0x9E37_79B9);
impl_word!(
    u64,
    typenum::U8,
    0xB7E1_5162_8AED_2A6B,
    0x9E37_79B9_7F4A_7C15
);
impl_word!(
    u128,
    typenum::U16,
    0xB7E1_5162_8AED_2A6A_BF71_5880_9CF4_F3C7,
    0x9E37_79B9_7F4A_7C15_F39C_C060_5CED_C835
);
