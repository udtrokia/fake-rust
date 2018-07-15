extern crate ed25519_dalek;
extern crate rand;
extern crate sha2;

use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
use ed25519_dalek::{KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};
use rand::OsRng;
use sha2::Sha512;

pub fn generate_keypair() -> [u8; KEYPAIR_LENGTH] {
    let mut cspring: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut cspring);
    keypair.to_bytes()
}

pub fn generate_publickey_from_secretkey(
    secret_key_bytes: &[u8; SECRET_KEY_LENGTH],
) -> [u8; PUBLIC_KEY_LENGTH] {
    let secret_key: SecretKey = SecretKey::from_bytes(secret_key_bytes).unwrap();
    let public_key = PublicKey::from_secret::<Sha512>(&secret_key);
    public_key.to_bytes()
}

pub fn signature(
    message: &str,
    secret_key_bytes: &[u8; SECRET_KEY_LENGTH],
) -> [u8; SIGNATURE_LENGTH] {
    let public_key_bytes = generate_publickey_from_secretkey(secret_key_bytes);
    let mut keypair_bytes: [u8; KEYPAIR_LENGTH] = [0u8; KEYPAIR_LENGTH];
    keypair_bytes[..SECRET_KEY_LENGTH].copy_from_slice(secret_key_bytes);
    keypair_bytes[SECRET_KEY_LENGTH..].copy_from_slice(&public_key_bytes);
    let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
    let signature: Signature = keypair.sign::<Sha512>(message.as_bytes());
    signature.to_bytes()
}

pub fn verify(
    message: &str,
    signature_bytes: &[u8; SIGNATURE_LENGTH],
    public_key_bytes: &[u8; PUBLIC_KEY_LENGTH],
) -> bool {
    let public_key: PublicKey = PublicKey::from_bytes(public_key_bytes).unwrap();
    let signature: Signature = Signature::from_bytes(signature_bytes).unwrap();
    public_key.verify::<Sha512>(message.as_bytes(), &signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 131, 80, 152, 32,  197, 122, 233, 218, 53, 206,
    // 210, 30, 249, 40,  137, 1,   181, 31,  51, 220,
    // 248, 73, 170, 167, 107, 229, 177, 118, 196, 1,
    // 205, 91 == secret_key
    // 144, 152, 169, 105, 199, 26,  237, 46,  176, 123,
    // 69,  124, 151, 67,  31,  170, 215, 213, 138, 159,
    // 10,  159, 243, 217, 79,  199, 13,  137, 174, 225,
    // 5, 121 == public_key

    #[test]
    fn test_generate_keypair() {
        assert_eq!(64, generate_keypair().len())
    }

    #[test]
    fn test_generate_public_from_secretkey() {
        let secret_key_bytes: [u8; 32] = [
            131, 80, 152, 32, 197, 122, 233, 218, 53, 206, 210, 30, 249, 40, 137, 1, 181, 31, 51,
            220, 248, 73, 170, 167, 107, 229, 177, 118, 196, 1, 205, 91,
        ];

        let public_key_bytes: [u8; 32] = [
            144, 152, 169, 105, 199, 26, 237, 46, 176, 123, 69, 124, 151, 67, 31, 170, 215, 213,
            138, 159, 10, 159, 243, 217, 79, 199, 13, 137, 174, 225, 5, 121,
        ];

        assert_eq!(
            public_key_bytes,
            generate_publickey_from_secretkey(&secret_key_bytes)
        );
    }

    #[test]
    fn test_signature() {
        let secret_key_bytes: [u8; 32] = [
            131, 80, 152, 32, 197, 122, 233, 218, 53, 206, 210, 30, 249, 40, 137, 1, 181, 31, 51,
            220, 248, 73, 170, 167, 107, 229, 177, 118, 196, 1, 205, 91,
        ];

        let signature_bytes: [u8; 64] = [
            234, 9, 15, 117, 43, 121, 110, 206, 139, 55, 54, 116, 106, 33, 100, 212, 157, 98, 65,
            241, 103, 219, 254, 207, 158, 49, 51, 206, 4, 32, 150, 200, 43, 192, 225, 172, 33, 25,
            97, 46, 96, 169, 63, 208, 227, 162, 35, 180, 248, 243, 139, 74, 154, 181, 22, 75, 122,
            178, 52, 93, 154, 197, 13, 10,
        ];

        assert_eq!(
            signature_bytes[..],
            signature("test", &secret_key_bytes)[..]
        );
    }

    #[test]
    fn test_verify() {
        let public_key_bytes: [u8; 32] = [
            144, 152, 169, 105, 199, 26, 237, 46, 176, 123, 69, 124, 151, 67, 31, 170, 215, 213,
            138, 159, 10, 159, 243, 217, 79, 199, 13, 137, 174, 225, 5, 121,
        ];

        let signature_bytes: [u8; 64] = [
            234, 9, 15, 117, 43, 121, 110, 206, 139, 55, 54, 116, 106, 33, 100, 212, 157, 98, 65,
            241, 103, 219, 254, 207, 158, 49, 51, 206, 4, 32, 150, 200, 43, 192, 225, 172, 33, 25,
            97, 46, 96, 169, 63, 208, 227, 162, 35, 180, 248, 243, 139, 74, 154, 181, 22, 75, 122,
            178, 52, 93, 154, 197, 13, 10,
        ];

        assert_eq!(true, verify("test", &signature_bytes, &public_key_bytes));
        assert_eq!(false, verify("test1", &signature_bytes, &public_key_bytes));
    }
}
