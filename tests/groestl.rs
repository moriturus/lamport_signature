#![feature(test)]

extern crate groestl;
extern crate lamport_signature;
extern crate rand;
extern crate test;
#[macro_use]
extern crate log;
extern crate env_logger;

use groestl::{Groestl256, Groestl512};
use lamport_signature::PrivateKey;
use rand::OsRng;

#[test]
fn test_sign_then_verify_groestl_256_private_key() {
    const DATA: &'static [u8] = b"hello, world!";

    let _ = env_logger::try_init();

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Groestl256>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    debug!("signature: {:?}", signature);
    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));
}

#[test]
fn test_sign_then_verify_groestl_512_private_key() {
    const DATA: &'static [u8] = b"hello, world!";

    let _ = env_logger::try_init();

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Groestl512>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    debug!("signature: {:?}", signature);
    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));
}

#[test]
#[should_panic]
fn test_sign_then_verify_groestl_256_private_key_reuse_should_panic() {
    const DATA: &'static [u8] = b"hello, world!";
    const SECOND_DATA: &'static [u8] = b"bello, vorld?";

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Groestl256>::new(&mut rng);
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
fn test_sign_then_verify_groestl_512_private_key_reuse_should_panic() {
    const DATA: &'static [u8] = b"hello, world!";
    const SECOND_DATA: &'static [u8] = b"bello, vorld?";

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Groestl512>::new(&mut rng);
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
