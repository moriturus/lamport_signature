#![feature(test)]

extern crate blake2;
extern crate lamport_signature;
extern crate rand;
extern crate test;

use blake2::{Blake2b, Blake2s};
use lamport_signature::PrivateKey;
use rand::OsRng;
use test::Bencher;

#[bench]
fn bench_sign_then_verify_blake2s_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Blake2s>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_blake2b_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Blake2b>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}
