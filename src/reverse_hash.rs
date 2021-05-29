use reccak::{hash, Digest, Input, permutations};
use std::thread;

const CHARS: &[u8] =
    b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#%^-_=+([{<)]}>";

const DIGESTS: &[(Digest, usize)] = &[
    // (
    //     [
    //         0xCFEA, 0xCDDA, 0xA7B4, 0x9BC7, 0x435C, 0x2564, 0x10DF, 0x11ED,
    //     ],
    //     2,
    // ),
    // (
    //     [
    //         0x46E1, 0x4669, 0x6C40, 0x8A28, 0xD1F6, 0xBBB1, 0x635D, 0xCAC0,
    //     ],
    //     3,
    // ),
    (
        [
            0xCCC0, 0x9636, 0x70A4, 0xC12F, 0x0745, 0x028B, 0x267F, 0x4AE5,
        ],
        4,
    ),
];

fn worker<'a>(permutations: impl Iterator<Item = Input>, expected_digest: Digest) {
    for permutation in permutations {
        let calculated_digest = hash(permutation.clone().into());
        if calculated_digest == expected_digest {
            println!(
                "found matching permutation: `{}` for digest: `{:X?}`",
                std::str::from_utf8(&permutation).unwrap(),
                calculated_digest,
            );
            std::process::exit(0);
        }
    }
}

fn main() {
    let cpus = num_cpus::get();
    for (expected_digest, input_size) in DIGESTS {
        println!(
            "starting search for {:?} with input size: {}",
            expected_digest, *input_size
        );
        let permutations = permutations(CHARS, *input_size);
        let chunk_size = CHARS.len().pow(*input_size as u32) / cpus;
        let cpus = 0..cpus;
        let handles = cpus
            .map(move |i| {
                println!("spawning worker {}", i);
                let skipped = i * chunk_size;
                let permutations = permutations.clone();
                thread::spawn(move || {
                    worker(
                        permutations.skip(skipped).take(chunk_size),
                        expected_digest.clone(),
                    );
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

