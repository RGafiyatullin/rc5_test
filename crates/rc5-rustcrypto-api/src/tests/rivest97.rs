use super::common::run_case;

use crate::rivest97::RC5_32_12_16;

#[test]
fn ex_1() {
    run_case::<RC5_32_12_16>(
        "00000000000000000000000000000000",
        "0000000000000000",
        "21a5dbee154b8f6d",
    );
}

#[test]
fn ex_2() {
    run_case::<RC5_32_12_16>(
        "915f4619be41b2516355a50110a9ce91",
        "21a5dbee154b8f6d",
        "f7c013ac5b2b8952",
    );
}

#[test]
fn ex_3() {
    run_case::<RC5_32_12_16>(
        "783348e75aeb0f2fd7b169bb8dc16787",
        "f7c013ac5b2b8952",
        "2f42b3b70369fc92",
    );
}

#[test]
fn ex_4() {
    run_case::<RC5_32_12_16>(
        "dc49db1375a5584f6485b413b5f12baf",
        "2f42b3b70369fc92",
        "65c178b284d197cc",
    );
}

#[test]
fn ex_5() {
    run_case::<RC5_32_12_16>(
        "5269f149d41ba0152497574d7f153125",
        "65c178b284d197cc",
        "eb44e415da319824",
    );
}
