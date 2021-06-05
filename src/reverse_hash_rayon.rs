use reccak::reverse_hash::{CHARS, DIGESTS};
use reccak::permutations;
use std::time::Instant;

fn main() {
    for &(expected_digest, input_size) in DIGESTS {
        use rayon::prelude::*;
        let start = Instant::now();
        let p = permutations(CHARS, input_size).par_bridge().find_any(|p| {
            if expected_digest == reccak::hash(p.clone()) {
                true
            } else {
                false
            }
        });
        println!(
            "Reversed hash {:X?}, input is: `{}`, took {:?}",
            expected_digest,
            std::str::from_utf8(p.unwrap().as_slice()).unwrap(),
            start.elapsed()
        )
    }
}
