use std::fs;
use hex_literal::hex;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// At the time of https://twitter.com/PolytopeLabs/status/1626211907643506693 Randao Revel is https://beaconscan.com/slot/5811141
const SEED: [u8; 96] = hex!("b0e376e2c805c707a7bec9d5fc38fa9d095c6a083f80efaf7e6131ffad7eaf8a3fa331f5a0eba47140b7b118d122f3f7054ef262e9cd71da6a1b339cd9c0cbb04742d573d7ee2873041b75077a1f51ecd1e7b57f358e8f53a1d600fc6f9b8b54");

fn read_file() -> Vec<String> {
    let contents = fs::read_to_string("participants.txt").unwrap();

    contents.split_whitespace()
    .collect::<Vec<_>>()
    .iter()
    .map(|s| s.to_string())
    .collect()

}

fn sub_sample<T>(set: &[T], k: usize) -> Vec<&T> {
    let mut seed = [0u8;32];
    seed.copy_from_slice(&SEED[..32]);

    let mut rng = StdRng::from_seed(seed);
    let mut sub_sample = Vec::with_capacity(k);

    for i in 0..set.len() {
        if i < k { 

            sub_sample.push(&set[i]);
        } else {
            let j = rng.gen_range(0..=i+1);

            if j < k {
                
                sub_sample[j] = &set[i];
            }
        }
    }

    sub_sample
}


fn main() {
    let sample = read_file();
    let selection = sub_sample(&sample, 5);

    println!("{:?}", selection);
}
