#![feature(test)]

extern crate lamport_signature;
extern crate rand;
extern crate sha2;
extern crate test;

use lamport_signature::PrivateKey;
use rand::OsRng;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};
use test::Bencher;

#[bench]
fn bench_sign_then_verify_sha2_224_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha224>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha2_256_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha256>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha2_384_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha384>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha2_512_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha512>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha2_512_trunc_224_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha512Trunc224>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}

#[bench]
fn bench_sign_then_verify_sha2_512_trunc_256_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Sha512Trunc256>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}
