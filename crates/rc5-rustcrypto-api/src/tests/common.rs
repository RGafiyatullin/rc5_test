use cipher::{BlockDecrypt, BlockEncrypt, BlockSizeUser, KeyInit};
use generic_array::GenericArray;

pub fn run_case<Cipher>(key_hex: &str, plaintext_hex: &str, ciphertext_hex: &str)
where
    Cipher: KeyInit + BlockSizeUser + BlockEncrypt + BlockDecrypt,
{
    let key = hex::decode(key_hex).expect("key hex::decode failed");
    let plaintext = hex::decode(plaintext_hex).expect("plaintext hex::decode failed");
    let ciphertext = hex::decode(ciphertext_hex).expect("plaintext hex::decode failed");

    let cipher = <Cipher as KeyInit>::new_from_slice(key.as_ref()).expect("KeyInit failed");

    let mut block = GenericArray::clone_from_slice(plaintext.as_ref());

    <Cipher as BlockEncrypt>::encrypt_block(&cipher, &mut block);
    assert_eq!(block.as_ref(), ciphertext);

    <Cipher as BlockDecrypt>::decrypt_block(&cipher, &mut block);
    assert_eq!(block.as_ref(), plaintext);
}
