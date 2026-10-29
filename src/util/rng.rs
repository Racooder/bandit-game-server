const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[inline]
fn combine(h: u64, v: i32) -> u64 {
    mix64(h.wrapping_add(GAMMA) ^ (v as u32 as u64))
}

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: i32) -> Self {
        Self {
            state: combine(0, seed),
        }
    }

    pub fn from_parts(parts: &[i32]) -> Self {
        Self {
            state: parts.iter().fold(0, |h, &v| combine(h, v)),
        }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GAMMA);
        mix64(self.state)
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 * (1.0 / (1u32 << 24) as f32)
    }

    #[inline]
    pub fn next_below(&mut self, n: u32) -> u32 {
        ((self.next_u32() as u64 * n as u64) >> 32) as u32
    }
}
