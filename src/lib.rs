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
        struct TestData<'a> {
            message: &'static str,
            digest: &'a [u16],
        }
        const DATA: &[TestData] = &[
            TestData {
                message: "",
                digest: &[
                    0xE225, 0x5BFB, 0xD3CF, 0x86E0, 0xDBE5, 0x2AA9, 0x6782, 0xEB8D,
                ],
            },
            TestData {
                message: "AbCxYz",
                digest: &[
                    0x5A0F, 0xB1F1, 0xF014, 0x9827, 0xC536, 0x280F, 0xEAD1, 0x67D1,
                ],
            },
            TestData {
                message: "1234567890",
                digest: &[
                    0x3746, 0x689D, 0x2ED8, 0x0406, 0xEBE2, 0x038B, 0x5FDD, 0xF9D5,
                ],
            },
            // TestData {
            //     message: "Ala ma kota, kot ma ale.",
            //     digest: &[
            //         0xD662, 0xF8E0, 0x328D, 0x46CB, 0x53CC, 0xB89D, 0x219A, 0x9485,
            //     ],
            // },
            // TestData {
            //     message: "Ty, ktory wchodzisz, zegnaj sie z nadzieja.",
            //     digest: &[
            //         0xB534, 0xF7EF, 0xF714, 0x8C43, 0x2057, 0xDFD6, 0x1138, 0x7A30,
            //     ],
            // },
            // TestData {
            //     message: "Litwo, Ojczyzno moja! ty jestes jak zdrowie;",
            //     digest: &[
            //         0x7FEC, 0x8BC2, 0x482B, 0xA864, 0xCA69, 0x9270, 0x5207, 0x3CDD,
            //     ],
            // },
        ];
        for data in DATA {
            let digest = hash(data.message.as_bytes());
            assert_eq!(
                digest, data.digest,
                "invalid digest received for message: `{}`",
                data.message
            );
        }
    }
}
