#![feature(test)]

extern crate lamport_signature;
extern crate rand;
extern crate test;
extern crate whirlpool;

use lamport_signature::PrivateKey;
use rand::OsRng;
use whirlpool::Whirlpool;

#[test]
fn test_sign_then_verify_whirlpool_private_key() {
    const DATA: &'static [u8] = b"hello, world!";

    let mut rng = OsRng::new().unwrap();
    let mut private_key = PrivateKey::<Whirlpool>::new(&mut rng);
    let public_key = private_key.public_key();

    let signature = private_key.sign(DATA);

    assert!(signature.is_ok());

    let signature = signature.unwrap();
    assert!(public_key.verify(&signature, DATA));
}
