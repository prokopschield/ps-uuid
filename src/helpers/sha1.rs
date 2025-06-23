#![allow(clippy::many_single_char_names)]
#![forbid(unsafe_code)]

use core::fmt;

use crate::ToHex;

#[must_use]
pub fn sha1(data: &[u8]) -> [u8; 20] {
    Sha1::digest(data)
}

#[derive(Clone)]
pub struct Sha1 {
    state: [u32; 5],
    len_bits: u64,
    buf: [u8; 64],
    buf_len: usize,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha1 {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: [
                0x6745_2301,
                0xefcd_ab89,
                0x98ba_dcfe,
                0x1032_5476,
                0xc3d2_e1f0,
            ],
            len_bits: 0,
            buf: [0; 64],
            buf_len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
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
    }

    #[must_use]
    pub fn finalize(mut self) -> [u8; 20] {
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
        self.buf[56..64].copy_from_slice(&self.len_bits.to_be_bytes());
        self.process_block(None);

        let mut out = [0u8; 20];
        for (i, &s) in self.state.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&s.to_be_bytes());
        }
        out
    }

    #[must_use]
    pub fn digest(data: &[u8]) -> [u8; 20] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }

    #[inline]
    fn process_block(&mut self, block: Option<&[u8]>) {
        let block = block.unwrap_or(&self.buf);

        debug_assert_eq!(block.len(), 64);

        let mut w = [0u32; 80];

        for (i, chunk) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes(
                #[allow(clippy::expect_used)]
                chunk
                    .try_into()
                    .expect(".chunk_exact(4) should yield four-byte slices"),
            );
        }

        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];

        #[allow(clippy::needless_range_loop)]
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5a82_7999),
                20..=39 => (b ^ c ^ d, 0x6ed_9eba1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1b_bcdc),
                _ => (b ^ c ^ d, 0xca62_c1d6),
            };

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }
}

impl fmt::Display for Sha1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.clone().finalize().to_hex())
    }
}

#[cfg(test)]
mod tests {
    use crate::{to_hex, ToHex};

    use super::Sha1;

    fn sha1_hex(s: &str) -> String {
        Sha1::digest(s.as_bytes()).to_hex()
    }

    #[test]
    fn fips_vectors() {
        let cases = [
            ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            ("abc", "a9993e364706816aba3e25717850c26c9cd0d89d"),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
            ),
            (
                "The quick brown fox jumps over the lazy dog",
                "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            ),
            (
                "The quick brown fox jumps over the lazy dog.",
                "408d94384216f890ff7a0c3528e8bed1e0b01621",
            ),
        ];
        for (input, expect) in cases {
            assert_eq!(sha1_hex(input), expect);
        }
    }

    #[test]
    fn incremental_vs_one_shot() {
        let data = b"fmt the fear and do it anyway!";
        let mut h = Sha1::new();
        for chunk in data.chunks(5) {
            h.update(chunk);
        }
        assert_eq!(to_hex(&h.finalize()), to_hex(&Sha1::digest(data)));
    }
}
