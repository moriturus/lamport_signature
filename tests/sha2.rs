#![feature(test)]

extern crate lamport_signature;
extern crate rand;
extern crate sha2;
extern crate test;
#[macro_use]
extern crate log;
extern crate env_logger;

use lamport_signature::PrivateKey;
use rand::OsRng;
use sha2::{Sha256, Sha512};

#[test]
fn test_sign_then_verify_sha2_256_private_key() {
    const DATA: &'static [u8] = b"hello, world!";

    let _ = env_logger::try_init();

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Sha256>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    debug!("signature: {:?}", signature);
    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));
}

#[test]
fn test_sign_then_verify_sha2_512_private_key() {
    const DATA: &'static [u8] = b"hello, world!";

    let _ = env_logger::try_init();

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Sha512>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    debug!("signature: {:?}", signature);
    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));
}

#[test]
#[should_panic]
fn test_sign_then_verify_sha2_256_private_key_reuse_should_panic() {
    const DATA: &'static [u8] = b"hello, world!";
    const SECOND_DATA: &'static [u8] = b"bello, vorld?";

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Sha256>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));

    let new_signature = private_key.sign(SECOND_DATA);

    assert!(new_signature.is_ok());

    let new_signature = new_signature.unwrap();
    assert!(public_key.verify(&new_signature, SECOND_DATA));
}

#[test]
#[should_panic]
fn test_sign_then_verify_sha2_512_private_key_reuse_should_panic() {
    const DATA: &'static [u8] = b"hello, world!";
    const SECOND_DATA: &'static [u8] = b"bello, vorld?";

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Sha512>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));

    let new_signature = private_key.sign(SECOND_DATA);

    assert!(new_signature.is_ok());

    let new_signature = new_signature.unwrap();
    assert!(public_key.verify(&new_signature, SECOND_DATA));
}
