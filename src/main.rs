use std::convert::TryInto;

const R: &[u16] = &[
    0x3EC2, 0x738D, 0xB119, 0xC5E7, 0x86C6, 0xDC1B, 0x57D6, 0xDA3A, 0x7710, 0x9200,
];

type Vector = [u16; 25];
type Matrix = [[u16; 5]; 5];
type Message = [u16; 10];

fn main() {
    let w: [u8; 20] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13,
    ];
    let w: [u16; 10] = w
        .chunks_exact(2)
        .map(|v| {
            let v1 = v[0];
            let v2 = v[1];
            u16::from_be_bytes([v1, v2])
        })
        .collect::<Vec<u16>>()
        .try_into()
        .unwrap();

    let mut a: [[u16; 5]; 5] = Default::default();
    let mut b: [[u16; 5]; 5] = Default::default();
    let mut c: [u16; 25] = Default::default();
    let mut d: [u16; 25] = Default::default();

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

    for i in 0..10 {
        round(&mut a, &mut b, &mut c, &mut d);
        iota(i, &mut a);
    }

    let digest = digest(&mut a, &mut b, &mut c, &mut d);
    for e in digest.iter() {
        let bytes = e.to_be_bytes();
        println!("0x{:X}", bytes[0]);
        println!("0x{:X}", bytes[1]);
    }
}

fn digest(a: &mut Matrix, b: &mut Matrix, c: &mut Vector, d: &mut Vector) -> [u16; 8] {
    let mut digest: [u16; 8] = [0; 8];

    digest[0] = a[0][0];
    digest[1] = a[0][1];
    digest[2] = a[0][2];
    digest[3] = a[0][3];
    digest[4] = a[0][4];
    digest[5] = a[0][0];
    round(a, b, c, d);
    digest[6] = a[0][1];
    digest[7] = a[0][2];
    // digest[8] = a[0][0];

    digest
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

fn round(a: &mut Matrix, b: &mut Matrix, c: &mut Vector, d: &mut Vector) {
    theta(a, c, d);
    rho(a);
    pi(a, b);
    chi(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let w: [u8; 20] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
        ];
        let w: [u16; 10] = w
            .chunks_exact(2)
            .map(|v| {
                let v1 = v[0];
                let v2 = v[1];
                u16::from_be_bytes([v1, v2])
            })
            .collect::<Vec<u16>>()
            .try_into()
            .unwrap();

        let mut a: [[u16; 5]; 5] = Default::default();
        let mut b: [[u16; 5]; 5] = Default::default();
        let mut c: [u16; 25] = Default::default();
        let mut d: [u16; 25] = Default::default();

        // let mut digest: [u16; 6] = [0; 6];

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

        for i in 0..10 {
            round(&mut a, &mut b, &mut c, &mut d);
            iota(i, &mut a);
        }

        const EXPECTED: [[u16; 5]; 5] = [
            [0x349F, 0x248F, 0xE791, 0x0E83, 0xD555],
            [0x40C8, 0x3AC3, 0xDBE2, 0x04E4, 0x3C40],
            [0x47F9, 0x2319, 0xD840, 0xEE29, 0x3B2B],
            [0x331C, 0xE945, 0x8660, 0x1B95, 0x72BA],
            [0xF1E1, 0xBDCD, 0x76CF, 0x6453, 0x4C68],
        ];
        assert_eq!(a, EXPECTED);
    }
}
