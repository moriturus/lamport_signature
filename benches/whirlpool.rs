#![feature(test)]

extern crate lamport_signature;
extern crate rand;
extern crate test;
extern crate whirlpool;

use lamport_signature::PrivateKey;
use rand::OsRng;
use test::Bencher;
use whirlpool::Whirlpool;

#[bench]
fn bench_sign_then_verify_whirlpool_private_key(b: &mut Bencher) {
    const DATA: &'static [u8] = b"hello, world!";

    b.iter(|| {
        let mut rng = OsRng::new().unwrap();
        let mut private_key = PrivateKey::<Whirlpool>::new(&mut rng);
        let public_key = private_key.public_key();

        let signature = private_key.sign(DATA).unwrap();
        public_key.verify(&signature, DATA)
    });
}
