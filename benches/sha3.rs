#![feature(test)]

extern crate lamport_signature;
extern crate rand;
extern crate sha3;
extern crate test;

use lamport_signature::PrivateKey;
use rand::OsRng;
use sha3::{Keccak224, Keccak256, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use test::Bencher;

#[bench]
fn bench_sign_then_verify_sha3_224_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha3_224>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha3_256_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha3_256>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha3_384_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha3_384>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha3_512_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha3_512>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_keccak_224_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Keccak224>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_keccak_256_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Keccak256>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_keccak_384_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Keccak384>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_keccak_512_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Keccak512>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}
