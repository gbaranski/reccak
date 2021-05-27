use crate::BLOCK_SIZE;

pub fn add_padding(input: &mut Vec<u8>) {
    let payload_size = input.len();
    let blocks = (payload_size + 1 - 1) / BLOCK_SIZE + 1;
    let bytes = blocks * BLOCK_SIZE;
    input.resize(bytes, 0u8);
    input[payload_size] = 0x80;
}

pub fn apply_padding(x: &[u8]) -> Vec<u8> {
    let blocks = ((x.len() as f32 + 1_f32) / BLOCK_SIZE as f32 + 1_f32).ceil() as usize;
    let mut y = Vec::with_capacity(blocks);
    y.extend_from_slice(x);
    y.extend_from_slice(&[0x80]);
    y.extend(std::iter::repeat(0).take(blocks * BLOCK_SIZE - y.len()));

    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_padding_delim_only() {
        let padded = apply_padding(&[0x80]);
        assert_eq!(padded, &[0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn apply_padding_single_block_small() {
        let padded = apply_padding(&[0x00, 0x01, 0x02, 0x03]);
        assert_eq!(padded, &[0x00, 0x01, 0x02, 0x03, 0x80, 0x00, 0x00, 0x00,]);
    }

    #[test]
    fn apply_padding_single_block() {
        let padded = apply_padding(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(padded, &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x80,]);
    }

    #[test]
    fn apply_padding_double_block_small() {
        let padded = apply_padding(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
        assert_eq!(
            padded,
            &[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]
        );
    }

    #[test]
    fn apply_padding_double_block() {
        let padded = apply_padding(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(
            padded,
            &[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]
        );
    }
}
