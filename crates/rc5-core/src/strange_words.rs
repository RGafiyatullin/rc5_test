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
                    //     eprintln!("\tASSERTION FAILED!");
                    //     eprintln!("\tm: {:24b}", MODULO);
                    //     eprintln!("\te: {:024b}", self.0);
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
                    // eprintln!("\tfrom_bytes");
                    // eprintln!("\th: {:02x}", b);

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
                    let out = out % MODULO;

                    #[cfg(test)]
                    {
                        eprintln!("\tadd");
                        // eprintln!("\tm: {:24b}", MODULO);
                        eprintln!("\tl: {:024b} | {:06x}", l, l);
                        eprintln!("\tr: {:024b} | {:06x}", r, r);
                        eprintln!("\to: {:024b} | {:06x}", out, out);
                    }

                    StrangeWord(out % MODULO).assert_sound()
                }

                fn sub(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let inv = MODULO - r;
                    let out = l + inv;
                    let out = out % MODULO;

                    #[cfg(test)]
                    {
                        eprintln!("\tsub");
                        // eprintln!("\tm: {:24b}", MODULO);
                        eprintln!("\tl: {:024b} | {:06x}", l, l);
                        eprintln!("\tr: {:024b} | {:06x}", r, r);
                        eprintln!("\ti: {:024b} | {:06x}", inv, inv);
                        eprintln!("\to: {:024b} | {:06x}", out, out);
                    }

                    StrangeWord(out).assert_sound()
                }

                fn rotl(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = {
                        let r = r % W;
                        (l << r) | (l >> (W - r))
                    };
                    let out = out % MODULO;

                    #[cfg(test)]
                    {
                        eprintln!("\trotl");
                        // eprintln!("\tm: {:24b}", MODULO);
                        eprintln!("\tl: {:024b} | {:06x}", l, l);
                        eprintln!("\tr: {:024b} | {:06x}", r, r);
                        eprintln!("\t : {} ({} mod {})", r, r % W, W);
                        eprintln!("\to: {:024b} | {:06x}", out, out);
                    }

                    StrangeWord(out).assert_sound()
                }

                fn rotr(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = {
                        let r = r % W;
                        (l >> r) | (l << (W - r))
                    };
                    let out = out % MODULO;

                    #[cfg(test)]
                    {
                        eprintln!("\trotr");
                        // eprintln!("\tm: {:24b}", MODULO);
                        eprintln!("\tl: {:024b} | {:06x}", l, l);
                        eprintln!("\tr: {:024b} | {:06x}", r, r);
                        eprintln!("\t : {} ({} mod {})", r, r % W, W);
                        eprintln!("\to: {:024b} | {:06x}", out, out);
                    }

                    StrangeWord(out).assert_sound()
                }

                fn xor(l: &StrangeWord, r: &StrangeWord) -> StrangeWord {
                    let l = l.assert_sound().0;
                    let r = r.assert_sound().0;
                    let out = l.bitxor(r);
                    let out = out % MODULO;

                    #[cfg(test)]
                    {
                        eprintln!("\txor");
                        // eprintln!("\tm: {:24b}", MODULO);
                        eprintln!("\tl: {:024b} | {:06x}", l, l);
                        eprintln!("\tr: {:024b} | {:06x}", r, r);
                        eprintln!("\to: {:024b} | {:06x}", out, out);
                    }

                    StrangeWord(out).assert_sound()
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
