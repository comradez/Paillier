mod keygen;
mod process;
use num_primes::BigUint;
use process::decrypt;
use process::encrypt;
fn main() {
    let (public_key, private_key) = keygen::gen_key(512_usize);
    // println!("public key: ({}, {})", public_key.0, public_key.1);
    // println!("private key: ({}, {})", private_key.0, private_key.1);
    let message = BigUint::from(3_usize);
    let secret  = encrypt(&message, &public_key, None);
    println!("message: {}", message);
    println!("secret: {}", &secret);
    println!("restored: {}", decrypt(&secret, &public_key, &private_key))    
    // println!("private key: {}", private_key);
}
