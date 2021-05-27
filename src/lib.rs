mod utils;

const R: &[u16] = &[
    0x3EC2, 0x738D, 0xB119, 0xC5E7, 0x86C6, 0xDC1B, 0x57D6, 0xDA3A, 0x7710, 0x9200,
];

type Digest = [u16; 8];
type Vector = [u16; 25];
type Matrix = [[u16; 5]; 5];

pub fn hash(w: &[u8]) -> Digest {
    let w = utils::apply_padding(w);
    let w = w
        .chunks_exact(2)
        .map(|v| u16::from_be_bytes([v[0], v[1]]))
        .collect::<Vec<u16>>();

    let (mut a, mut b, mut c, mut d) = Default::default();
    init_state(&mut a, &w);
    apply_all_rounds(&mut a, &mut b, &mut c, &mut d);

    let mut digest: Digest = [0; 8];
    digest[0..5].copy_from_slice(&a[0][0..5]);
    apply_all_rounds(&mut a, &mut b, &mut c, &mut d);
    digest[5..8].copy_from_slice(&a[0][0..3]);

    digest
}

fn apply_all_rounds(a: &mut Matrix, b: &mut Matrix, c: &mut Vector, d: &mut Vector) {
    for round in 0..10 {
        apply_round(round, a, b, c, d);
    }
}

fn apply_round(round: usize, a: &mut Matrix, b: &mut Matrix, c: &mut Vector, d: &mut Vector) {
    theta(a, c, d);
    rho(a);
    pi(a, b);
    chi(a, b);
    iota(round, a);
}

fn init_state(a: &mut Matrix, w: &[u16]) {
    for i in 0..5 {
        for j in 0..5 {
            let cv = if w.len() <= j + 5 * i {
                0
            } else {
                w[j + 5 * i]
            };
            a[i][j] = cv;
        }
    }
}

fn theta(a: &mut Matrix, c: &mut Vector, d: &mut Vector) {
    for i in 0..5 {
        c[i] = a[i][0] ^ a[i][1] ^ a[i][2] ^ a[i][3] ^ a[i][4];
    }

    for i in 0..5 {
        d[i] = c[(i as isize - 1).rem_euclid(5) as usize] ^ (c[(i + 1) % 5].rotate_left(1));
    }

    for i in 0..5 {
        for j in 0..5 {
            a[i][j] = a[i][j] ^ d[i];
        }
    }
}

fn rho(a: &mut Matrix) {
    for i in 0..5 {
        for j in 0..5 {
            a[i][j] = a[i][j].rotate_left((7 * i + j.rem_euclid(5)) as u32);
        }
    }
}

fn pi(a: &Matrix, b: &mut Matrix) {
    for i in 0..5 {
        for j in 0..5 {
            b[(3 * i + 2 * j) % 5][i] = a[i][j];
        }
    }
}

fn chi(a: &mut Matrix, b: &Matrix) {
    for i in 0..5 {
        for j in 0..5 {
            a[i][j] = b[i][j] ^ (!(b[(i + 1) % 5][j]) & b[(i + 2) % 5][j]);
        }
    }
}

fn iota(i: usize, a: &mut Matrix) {
    a[0][0] = a[0][0] ^ R[i];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        [
            (
                "",
                [
                    0xE2, 0x25, 0x5B, 0xFB, 0xD3, 0xCF, 0x86, 0xE0, 0xDB, 0xE5, 0x2A, 0xA9, 0x67,
                    0x82, 0xEB, 0x8D,
                ],
            ),
            (
                "AbCxYz",
                [
                    0x5A, 0x0F, 0xB1, 0xF1, 0xF0, 0x14, 0x98, 0x27, 0xC5, 0x36, 0x28, 0x0F, 0xEA,
                    0xD1, 0x67, 0xD1,
                ],
            ),
            (
                "1234567890",
                [
                    0x37, 0x46, 0x68, 0x9D, 0x2E, 0xD8, 0x04, 0x06, 0xEB, 0xE2, 0x03, 0x8B, 0x5F,
                    0xDD, 0xF9, 0xD5,
                ],
            ),
            (
                "Ala ma kota, kot ma ale.",
                [
                    0xD6, 0x62, 0xF8, 0xE0, 0x32, 0x8D, 0x46, 0xCB, 0x53, 0xCC, 0xB8, 0x9D, 0x21,
                    0x9A, 0x94, 0x85,
                ],
            ),
            (
                "Ty, ktory wchodzisz, zegnaj sie z nadzieja.",
                [
                    0xB5, 0x34, 0xF7, 0xEF, 0xF7, 0x14, 0x8C, 0x43, 0x20, 0x57, 0xDF, 0xD6, 0x11,
                    0x38, 0x7A, 0x30,
                ],
            ),
            (
                "a".repeat(48000).as_str(),
                [
                    0x07, 0x2F, 0xB0, 0x3B, 0xC3, 0xC9, 0x96, 0x50, 0x66, 0x3B, 0x2B, 0x89, 0xA6,
                    0xE9, 0x9F, 0x74,
                ],
            ),
            (
                "a".repeat(48479).as_str(),
                [
                    0xAA, 0x64, 0x8B, 0xAE, 0xF6, 0x95, 0x48, 0x33, 0xF9, 0x55, 0x5D, 0x55, 0xA7,
                    0x97, 0xD2, 0xCB,
                ],
            ),
            (
                "a".repeat(48958).as_str(),
                [
                    0x9A, 0x9C, 0x15, 0x4F, 0x81, 0x7A, 0x48, 0xE4, 0xE2, 0x8D, 0x8A, 0x8C, 0x68,
                    0x7A, 0xCD, 0x60,
                ],
            ),
        ]
        .iter()
        .for_each(|(message, expected_hash)| {
            let expected_hash = expected_hash
                .chunks_exact(2)
                .map(|v| u16::from_be_bytes([v[0], v[1]]))
                .collect::<Vec<u16>>();
            let hash = hash(message.as_bytes());
            assert_eq!(hash, expected_hash.as_slice(), "for message {}", message,);
        });
    }
}
