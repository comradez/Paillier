use num_primes::{Generator, BigUint, RandBigInt};
use num_bigint::{BigInt, ToBigInt};
use num::Integer;

pub fn gen_key(length: usize) -> ((BigUint, BigUint), (BigUint, BigUint)) {
    let p = Generator::new_prime(length);
    let q = Generator::new_prime(length);
    let one = BigUint::from(1_u32);
    println!("p is {}, q is {}", &p, &q);
    let n = &p * &q;
    let lambda = (&p - &one).lcm(&(&q - &one));
    let (g, mu) = gen_g(&n, &lambda);
    ((n, g), (lambda, mu))
}

pub fn l(x: &BigUint, n: &BigUint) -> BigUint {
    (x - BigUint::from(1_u32)) / n
}

pub fn quick_pow(base: &BigUint, exp: &BigUint, prime: &BigUint) -> BigUint {
    let mut temp_val = base.clone();
    let mut exp = exp.clone();
    let mut answer = BigUint::from(1_u32);
    let one = BigUint::from(1_u32);
    let two = BigUint::from(2_u32);
    while exp >= one {
        temp_val = temp_val % prime;
        if exp.is_odd() {
            answer *= &temp_val;
        }
        temp_val = &temp_val * &temp_val;
        exp /= &two;
    }
    answer % prime
}

fn gen_g(n: &BigUint, lambda: &BigUint) -> (BigUint, BigUint) {
    let n_sq = n * n;
    let one = BigInt::from(1_u32);
    loop {
        let g = rand::thread_rng().gen_biguint_below(&n_sq);
        let mu = l(&quick_pow(&g, lambda, &n_sq), n);
        let mu_signed = mu.to_bigint().unwrap();
        let n_signed = n.to_bigint().unwrap();
        let exgcd = mu_signed.extended_gcd(&n_signed);
        if exgcd.gcd == one {
            let mut x = exgcd.x;
            while x < BigInt::from(0) {
                x = x + &n_signed;
            }
            break (g, x.to_biguint().unwrap())
        }
    }
}