use std::{ops::{Shr, Sub}, env};

use eigentrust_zk::params::hasher::poseidon_bn254_5x5::Params;
use eigentrust_zk::poseidon::native::sponge::PoseidonSponge;
use eth_types::Field;
use halo2curves::{bn256::Fr, ff::PrimeField};
use primitive_types::U256;

const WIDTH: usize = 5;

fn main() {

    let difficulty_shift = env::args().nth(1).unwrap().parse::<i32>().unwrap();
    let args: Vec<String> = env::args().skip(2).collect();
    let inputs: Vec<Fr> = args.iter().map(|n| hex_to_field(n)).collect();

    // let inputs: Vec<Fr> = [
    //     "0x84426c1493c469f553446efdafb84ec9fd54aeed6d448e308e22434cb8d5ff4b",
    // ]
    // .iter()
    // .map(|n| hex_to_field(n))
    // .collect();

    let mut native_poseidon_sponge = PoseidonSponge::<Fr, WIDTH, Params>::new();
    native_poseidon_sponge.update(&inputs);
    let hash_result = native_poseidon_sponge.squeeze();

    let difficulty = (!U256::from(0)).shr(difficulty_shift);

    let hash_result_num = U256::from_little_endian(&hash_result.to_repr()[..]);

    if hash_result_num > difficulty {
        panic!("hash_result_num > difficulty");
    }

    println!("diff:{:?}, difficulty:{:?}", difficulty.sub(hash_result_num), difficulty);
}

/// Returns congruent field element for the given hex string.
pub fn hex_to_field<F: Field>(s: &str) -> F {
    let s = &s[2..];
    let mut bytes = hex::decode(s).expect("Invalid params");
    bytes.reverse();
    let mut bytes_wide: [u8; 64] = [0; 64];
    bytes_wide[..bytes.len()].copy_from_slice(&bytes[..]);
    F::from_uniform_bytes(&bytes_wide)
}

pub fn u256_to_field<F: Field>(num: &U256) -> F {
    let mut bytes_wide: [u8; 64] = [0; 64];
    num.to_little_endian(&mut bytes_wide[0..32]);
    F::from_uniform_bytes(&bytes_wide)
}