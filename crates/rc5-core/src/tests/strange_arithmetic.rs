strange_word_impl!(w8bits, u16, typenum::U1, 0b1110111, 0b1011111);

use w8bits::StrangeWord as W8bits;

use crate::{strange_words::StrangeArithmetics as A, traits::Arithmetics};

#[test]
fn t0() {
    let rots: &[(u16, u16, u16)] = &[
        (0b0000_0001, 0, 0b0000_0001),
        (0b0000_0001, 1, 0b0000_0010),
        (0b0000_0001, 2, 0b0000_0100),
        (0b0000_0001, 3, 0b0000_1000),
        (0b0000_0001, 4, 0b0001_0000),
        (0b0000_0001, 5, 0b0010_0000),
        (0b0000_0001, 6, 0b0100_0000),
        (0b0000_0001, 7, 0b1000_0000),
        (0b0000_0001, 8, 0b0000_0001),
    ];
    for &(a, r, b) in rots {
        let a: W8bits = a.into();
        let r: W8bits = r.into();
        let b: W8bits = b.into();

        // eprintln!("{:08b}:{}:{:08b}", u16::from(a), u16::from(r), u16::from(b));

        assert_eq!(
            A::rotl(&a, &r),
            b,
            "rotl({:08b}, {}) -> {:08b} [exp: {:08b}]",
            u16::from(a),
            r,
            u16::from(A::rotl(&a, &r)),
            u16::from(b)
        );
        assert_eq!(
            A::rotr(&b, &r),
            a,
            "rotr({:08b}, {}) -> {:08b} [exp: {:08b}]",
            u16::from(b),
            r,
            u16::from(A::rotl(&b, &r)),
            u16::from(a)
        );
    }
}
