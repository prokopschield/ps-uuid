#![allow(clippy::many_single_char_names)]
#![forbid(unsafe_code)]

use core::fmt;

use crate::ToHex;

#[must_use]
pub fn md5(data: &[u8]) -> [u8; 16] {
    Md5::digest(data)
}

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

#[derive(Clone)]
pub struct Md5 {
    state: [u32; 4],
    len_bits: u64,
    buf: [u8; 64],
    buf_len: usize,
}

impl Default for Md5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Md5 {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476],
            len_bits: 0,
            buf: [0; 64],
            buf_len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) -> &mut Self {
        self.len_bits = self.len_bits.wrapping_add((data.len() as u64) << 3);

        if self.buf_len > 0 {
            let n = (64 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + n].copy_from_slice(&data[..n]);
            self.buf_len += n;
            data = &data[n..];
            if self.buf_len == 64 {
                self.process_block(None);
                self.buf_len = 0;
            }
        }

        while data.len() >= 64 {
            self.process_block(Some(&data[..64]));
            data = &data[64..];
        }

        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len();
        }

        self
    }

    #[must_use]
    pub fn finalize(mut self) -> [u8; 16] {
        // padding: 0x80 + zeros + length LE
        self.buf[self.buf_len] = 0x80;
        self.buf_len += 1;

        if self.buf_len > 56 {
            for b in &mut self.buf[self.buf_len..] {
                *b = 0;
            }
            self.process_block(None);
            self.buf_len = 0;
        }

        for b in &mut self.buf[self.buf_len..56] {
            *b = 0;
        }

        self.buf[56..64].copy_from_slice(&self.len_bits.to_le_bytes());
        self.process_block(None);

        let mut out = [0u8; 16];

        for (i, &s) in self.state.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&s.to_le_bytes());
        }

        out
    }

    #[must_use]
    pub fn digest(data: &[u8]) -> [u8; 16] {
        let mut hasher = Self::new();

        hasher.update(data);

        hasher.finalize()
    }

    #[inline]
    fn process_block(&mut self, block: Option<&[u8]>) {
        let block = block.unwrap_or(&self.buf);

        debug_assert_eq!(block.len(), 64);

        let mut m = [0u32; 16];

        for (i, chunk) in block.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes(
                #[allow(clippy::expect_used)]
                chunk
                    .try_into()
                    .expect(".chunk_exact(4) should yield four-byte slices"),
            );
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        for i in 0..64 {
            let (mut f, g);
            if i < 16 {
                f = (b & c) | (!b & d);
                g = i;
            } else if i < 32 {
                f = (d & b) | (!d & c);
                g = (5 * i + 1) & 15;
            } else if i < 48 {
                f = b ^ c ^ d;
                g = (3 * i + 5) & 15;
            } else {
                f = c ^ (b | !d);
                g = (7 * i) & 15;
            }
            f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }
}

impl fmt::Display for Md5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.clone().finalize().to_hex())
    }
}

/* ------------------------------ tests --------------------------- */

#[cfg(test)]
mod tests {
    use crate::to_hex;

    use super::Md5;

    fn md5_hex(s: &str) -> String {
        to_hex(&Md5::digest(s.as_bytes()))
    }

    #[test]
    fn rfc_vectors() {
        let cases = [
            ("", "d41d8cd98f00b204e9800998ecf8427e"),
            ("a", "0cc175b9c0f1b6a831c399e269772661"),
            ("abc", "900150983cd24fb0d6963f7d28e17f72"),
            ("message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
            (
                "abcdefghijklmnopqrstuvwxyz",
                "c3fcd3d76192e4007dfb496cca67e13b",
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                "d174ab98d277d9f5a5611c2c9f419d9f",
            ),
            (
                "1234567890123456789012345678901234567890123456789012345678\
                     9012345678901234567890",
                "57edf4a22be3c955ac49da2e2107b67a",
            ),
        ];
        for (input, expect) in cases {
            assert_eq!(md5_hex(input), expect);
        }
    }

    #[test]
    fn incremental_vs_one_shot() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let mut h = Md5::new();
        for chunk in data.chunks(7) {
            h.update(chunk);
        }
        assert_eq!(to_hex(&h.finalize()), to_hex(&Md5::digest(data)));
    }
}
