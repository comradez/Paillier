mod keygen;
mod process;

use std::num::ParseIntError;
use num_primes::BigUint;
use process::decrypt;
use process::encrypt;

fn read_u32() -> Result<u32, ParseIntError> {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    n.trim().parse()
}

fn main() {
    let (public_key, private_key) = keygen::gen_key(64_usize);
    println!("Pleas input user number:");
    let n = read_u32().unwrap();
    let mut vote_sum = 0u32;
    let mut encrypted_vote_prod: BigUint = 1u32.into();

    for i in 0 .. n {
        println!("User {}, please enter message:", i);
        let vote = read_u32().unwrap();
        vote_sum += vote;
        let encrypted_vote = encrypt(&vote.into(), &public_key, None);
        encrypted_vote_prod *= &encrypted_vote;
        assert_eq!(decrypt(&encrypted_vote, &public_key, &private_key), vote.into());
    }

    let decrypted_vote_sum = decrypt(&encrypted_vote_prod, &public_key, &private_key);
    assert_eq!(decrypted_vote_sum, vote_sum.into());
    println!("Decrypted message sum is {}, result validated.", decrypted_vote_sum);
}
