pub enum StrangeArithmetics {}
pub enum StrangeByteRepr {}
pub enum StrangeMagic {}

macro_rules! strange_word_impl {
    ($mod: ident, $Container: ty, $ByteLen: ty, $P: literal, $Q: literal) => {
        mod $mod {

            use core::ops::BitXor;

            use typenum::Unsigned;
            use $crate::strange_words::{StrangeArithmetics, StrangeByteRepr, StrangeMagic};

            use $crate::traits::{Arithmetics, Magic, WordByteRepr, WordBytes, WordSize};

            const W: $Container = (<$ByteLen>::USIZE * 8) as $Container;
            const MODULO: $Container = 0b1 << W;

            #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
            pub struct StrangeWord($Container);

            impl From<$Container> for StrangeWord {
                fn from(v: $Container) -> Self {
                    Self(v).assert_sound()
                }
            }
            impl From<StrangeWord> for $Container {
                fn from(v: StrangeWord) -> Self {
                    v.0
                }
            }
            impl core::fmt::Display for StrangeWord {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    core::fmt::Display::fmt(&self.0, f)
                }
            }

            impl StrangeWord {
                const fn assert_sound(self) -> Self {
                    // if self.0 >= MODULO {
                    //     eprintln!("ASSERTION FAILED!");
                    //     eprintln!("m: {:16b}", MODULO);
                    //     eprintln!("e: {:016b}", self.0);
                    // }

                    assert!(self.0 < MODULO);

                    self
                }
            }

            impl WordSize for StrangeWord {
                type ByteLen = $ByteLen;
            }

            impl Magic<StrangeWord> for StrangeMagic {
                const THREE: StrangeWord = StrangeWord(3);
                const P: StrangeWord = StrangeWord($P).assert_sound();
                const Q: StrangeWord = StrangeWord($Q).assert_sound();
            }

            impl WordByteRepr<StrangeWord> for StrangeByteRepr {
                fn from_bytes(b: &WordBytes<StrangeWord>) -> StrangeWord {
                    // eprintln!("from_bytes");
                    // eprintln!("h: {:02x}", b);

                    let mut dummy = <$Container>::to_le_bytes(0);
                    dummy[..<StrangeWord as WordSize>::ByteLen::USIZE].copy_from_slice(b.as_ref());
                    StrangeWord(<$Container>::from_le_bytes(dummy)).assert_sound()
                }

                fn to_bytes(w: &StrangeWord) -> WordBytes<StrangeWord> {
                    let bytes = w.assert_sound().0.to_le_bytes();
                    WordBytes::<StrangeWord>::clone_from_slice(
                        &bytes[..<StrangeWord as WordSize>::ByteLen::USIZE],
                    )
                }
            }

            impl Arithmetics<StrangeWord> for StrangeArithmetics {
                fn add(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = l + r;

                    // #[cfg(test)]
                    // {
                    //     eprintln!("add");
                    //     eprintln!("m: {:16b}", MODULO);
                    //     eprintln!("l: {:016b}", l);
                    //     eprintln!("r: {:016b}", r);
                    //     eprintln!("o: {:016b}", out);
                    // }

                    StrangeWord(out % MODULO).assert_sound()
                }

                fn sub(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let inv = MODULO - r;
                    let out = l + inv;

                    // #[cfg(test)]
                    // {
                    //     eprintln!("sub");
                    //     eprintln!("m: {:16b}", MODULO);
                    //     eprintln!("l: {:016b}", l);
                    //     eprintln!("r: {:016b}", r);
                    //     eprintln!("i: {:016b}", inv);
                    //     eprintln!("o: {:016b}", out);
                    // }

                    StrangeWord(out % MODULO).assert_sound()
                }

                fn rotl(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = (l << (r & (W - 1))) | (l >> (W - (r & (W - 1))));

                    // #[cfg(test)]
                    // {
                    //     eprintln!("rotl");
                    //     eprintln!("m: {:16b}", MODULO);
                    //     eprintln!("l: {:016b}", l);
                    //     eprintln!("r: {:016b}", r);
                    //     eprintln!(" : {}", r);
                    //     eprintln!("o: {:016b}", out);
                    // }

                    StrangeWord(out % MODULO).assert_sound()
                }

                fn rotr(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = (l >> (r & (W - 1))) | (l << (W - (r & (W - 1))));

                    // #[cfg(test)]
                    // {
                    //     eprintln!("rotr");
                    //     eprintln!("m: {:16b}", MODULO);
                    //     eprintln!("l: {:016b}", l);
                    //     eprintln!("r: {:016b}", r);
                    //     eprintln!(" : {}", r);
                    //     eprintln!("o: {:016b}", out);
                    // }

                    StrangeWord(out % MODULO).assert_sound()
                }

                fn xor(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = l.bitxor(r);

                    // #[cfg(test)]
                    // {
                    //     eprintln!("xor");
                    //     eprintln!("m: {:16b}", MODULO);
                    //     eprintln!("l: {:016b}", l);
                    //     eprintln!("r: {:016b}", r);
                    //     eprintln!("o: {:016b}", out);
                    // }

                    StrangeWord(out % MODULO).assert_sound()
                }
            }
        }
    };
}

strange_word_impl!(w24bit, u32, typenum::U3, 0xB7E151, 0x9E3779);
strange_word_impl!(w32bit, u64, typenum::U4, 0xB7E1_5163, 0x9E37_79B9);
strange_word_impl!(
    w80bit,
    u128,
    typenum::U10,
    0xB7E1_5162_8AED_2A6A_BF71,
    0x9E37_79B9_7F4A_7C15_F39D
);

pub use w24bit::StrangeWord as W24bit;
pub use w32bit::StrangeWord as W32bit;
pub use w80bit::StrangeWord as W80bit;
