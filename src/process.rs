use num_primes::{RandBigInt, BigUint};

use crate::keygen::l;

pub fn encrypt(message: &BigUint, key: &(BigUint, BigUint), r: Option<BigUint>) -> BigUint {
    let (n, g) = key;
    let r = r.map_or_else(|| {
        rand::thread_rng().gen_biguint_below(n)
    }, |r| r % n);
    let n_sq = n * n;
    g.modpow(message, &n_sq) * r.modpow(n, &n_sq) % n_sq
}

pub fn decrypt(secret: &BigUint, public_key: &(BigUint, BigUint), private_key: &(BigUint, BigUint)) -> BigUint {
    let (n, _) = public_key;
    let (lambda, mu) = private_key;
    let n_sq = n * n;
    l(&secret.modpow(lambda, &n_sq), n) * mu % n
}