use num_primes::{RandBigInt, BigUint};

use crate::keygen::{l, quick_pow};

pub fn encrypt(message: &BigUint, key: &(BigUint, BigUint), r: Option<BigUint>) -> BigUint {
    let (n, g) = key;
    let r = r.map_or_else(|| {
        rand::thread_rng().gen_biguint_below(n)
    }, |r| r % n);
    println!("r is {}", &r);
    let n_sq = n * n;
    quick_pow(&g, &message, &n_sq) * quick_pow(&r, &n, &n_sq) % n_sq
}

pub fn decrypt(secret: &BigUint, public_key: &(BigUint, BigUint), private_key: &(BigUint, BigUint)) -> BigUint {
    let (n, _) = public_key;
    let (lambda, mu) = private_key;
    let n_sq = n * n;
    l(&quick_pow(secret, lambda, &n_sq), n) * mu % n
}