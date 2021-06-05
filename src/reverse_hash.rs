use crate::Digest;

pub const CHARS: &[u8] =
    b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#%^-_=+([{<)]}>";

pub const DIGESTS: &[(Digest, usize)] = &[
    (
        [
            0xCFEA, 0xCDDA, 0xA7B4, 0x9BC7, 0x435C, 0x2564, 0x10DF, 0x11ED,
        ],
        2,
    ),
    (
        [
            0x46E1, 0x4669, 0x6C40, 0x8A28, 0xD1F6, 0xBBB1, 0x635D, 0xCAC0,
        ],
        3,
    ),
    (
        [
            0xCCC0, 0x9636, 0x70A4, 0xC12F, 0x0745, 0x028B, 0x267F, 0x4AE5,
        ],
        4,
    ),
];

