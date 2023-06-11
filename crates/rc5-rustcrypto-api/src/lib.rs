#![no_std]

pub mod rc5;

pub mod rivest97 {
    type RC5_32_12<B> = crate::rc5::RC5<u32, typenum::U12, B>;
    type RC5_64_16<B> = crate::rc5::RC5<u64, typenum::U16, B>;

    pub type RC5_32_12_16 = RC5_32_12<typenum::U16>;
    pub type RC5_32_12_24 = RC5_32_12<typenum::U24>;
    pub type RC5_32_12_32 = RC5_32_12<typenum::U32>;

    pub type RC5_64_16_16 = RC5_64_16<typenum::U16>;
    pub type RC5_64_16_24 = RC5_64_16<typenum::U24>;
    pub type RC5_64_16_32 = RC5_64_16<typenum::U32>;
}

pub mod krovetz18 {
    type RC5_32_16<B> = crate::rc5::RC5<u32, typenum::U16, B>;
    type RC5_64_20<B> = crate::rc5::RC5<u64, typenum::U20, B>;

    pub type RC5_32_16_16 = RC5_32_16<typenum::U16>;
    pub type RC5_32_16_24 = RC5_32_16<typenum::U24>;
    pub type RC5_32_16_32 = RC5_32_16<typenum::U32>;

    pub type RC5_64_20_16 = RC5_64_20<typenum::U16>;
    pub type RC5_64_20_24 = RC5_64_20<typenum::U24>;
    pub type RC5_64_20_32 = RC5_64_20<typenum::U32>;
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;
