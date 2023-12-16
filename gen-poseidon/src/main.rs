use std::env;
use std::marker::PhantomData;

use halo2_gadgets::poseidon::primitives::{self as poseidon, ConstantLength};
use halo2_proofs::halo2curves::pasta::Fp;
use halo2_proofs::arithmetic::FieldExt;
use halo2_gadgets::poseidon::primitives::*;


#[derive(Debug, Clone, Copy)]
pub struct MySpec<F: FieldExt, const WIDTH: usize, const RATE: usize>{
    _marker: PhantomData<F>
}

impl<F: FieldExt, const WIDTH: usize, const RATE: usize> Spec<F, WIDTH, RATE> for MySpec<F, WIDTH, RATE> {
    fn full_rounds() -> usize {
        8
    }

    fn partial_rounds() -> usize {
        56
    }

    fn sbox(val: F) -> F {
        val.pow_vartime(&[5])
    }

    fn secure_mds() -> usize {
        0
    }
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let nums: Vec<u64> = args.into_iter()
        .map(|arg| arg.parse::<u64>().unwrap())
        .collect();
    
    const WIDTH: usize = 5;
    const RATE: usize = 4;
    const L: usize = 4;

    let mut hash_input = [Fp::from(0); 4];

    let mut i = 0;
    for input in &nums {
        hash_input[i] = Fp::from(input.clone());
        i = i + 1;
    }

    // target/release/gen-poseidon 13039933381888873592 199037045790087636 12545778012658302043 9761041669023303250
    // 0x0b0d649a21bfba7846a89a8b0f96d4b1d4e5d6fa5d0fa2220e138e11718d6571

    // 0x0b0d649a21bfba7846a89a8b0f96d4b1d4e5d6fa5d0fa2220e138e11718d6571
    
    // let input = 99u64;
    // let i1 = 13039933381888873592u64;
    // let i2 = 199037045790087636u64;
    // let i3 = 12545778012658302043u64;
    // let i4 = 9761041669023303250u64;
    // let hash_input = [
    //     // Fp::from(input),
    //     // Fp::from(input),
    //     // Fp::from(input),
    //     // Fp::from(input),
    //     Fp::from(i1),
    //     Fp::from(i2),
    //     Fp::from(i3),
    //     Fp::from(i4),
    //     // Fp::from(input),
    //     // Fp::from(input),
    // ];
    // // 8545665409054675217 10458395863604961410 11482625154744840039 1512345909812037551
    // // 0x37d87170eabb572d149983031509e3c04bdd776fc2e8abb57790757412ec53db
    // // 0x268706bbd4e334ffb5865526d24e1a5721147b72f9b01eb0a6d4fff0c2aa7759
    // const WIDTH: usize = 5;
    // const RATE: usize = 4;
    // const L: usize = 4;

    // compute the hash
    let digest =
        poseidon::Hash::<_, MySpec<Fp, WIDTH, RATE>, ConstantLength<L>, WIDTH, RATE>::init()
            .hash(hash_input);

    println!("{:?}", digest);
}
