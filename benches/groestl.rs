#![feature(test)]

extern crate groestl;
extern crate lamport_signature;
extern crate rand;
extern crate test;

use groestl::{Groestl224, Groestl256, Groestl384, Groestl512};
use lamport_signature::PrivateKey;
use rand::OsRng;
use test::Bencher;

#[bench]
fn bench_sign_then_verify_groestl_224_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Groestl224>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_groestl_256_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Groestl256>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_groestl_384_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Groestl384>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_groestl_512_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Groestl512>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}
