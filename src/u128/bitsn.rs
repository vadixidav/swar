use crate::*;
use core::ops::{BitAnd, Shr};

pub const LEFT_MASKS: [u128; 7] = [
    0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0000,
    0xFFFF_FFFF_0000_0000_FFFF_FFFF_0000_0000,
    0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000,
    0xFF00_FF00_FF00_FF00_FF00_FF00_FF00_FF00,
    0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0,
    0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC,
    0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA,
];

pub const RIGHT_MASKS: [u128; 7] = [
    0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF,
    0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF,
    0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF,
    0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF,
    0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F,
    0x3333_3333_3333_3333_3333_3333_3333_3333,
    0x5555_5555_5555_5555_5555_5555_5555_5555,
];

pub const ONES2: u128 = 0x5555_5555_5555_5555_5555_5555_5555_5555;
pub const ONES4: u128 = 0x1111_1111_1111_1111_1111_1111_1111_1111;
pub const ONES8: u128 = 0x0101_0101_0101_0101_0101_0101_0101_0101;
pub const ONES16: u128 = 0x0001_0001_0001_0001_0001_0001_0001_0001;
pub const ONES32: u128 = 0x0000_0001_0000_0001_0000_0001_0000_0001;
pub const ONES64: u128 = 0x0000_0000_0000_0001_0000_0000_0000_0001;

pub const SIGNS2: u128 = ONES2 << 1;
pub const SIGNS4: u128 = ONES4 << 3;
pub const SIGNS8: u128 = ONES8 << 7;
pub const SIGNS16: u128 = ONES16 << 15;
pub const SIGNS32: u128 = ONES32 << 31;
pub const SIGNS64: u128 = ONES64 << 63;

pub const WEIGHT_MASK2: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
pub const WEIGHT_MASK4: u128 = (ONES4 << 3) - ONES4;
pub const WEIGHT_MASK8: u128 = (ONES8 << 4) - ONES8;
pub const WEIGHT_MASK16: u128 = (ONES16 << 5) - ONES16;
pub const WEIGHT_MASK32: u128 = (ONES32 << 6) - ONES32;
pub const WEIGHT_MASK64: u128 = (ONES64 << 7) - ONES64;

pub const WEIGHT_MSB2: u128 = ONES2 << 1;
pub const WEIGHT_MSB4: u128 = ONES4 << 2;
pub const WEIGHT_MSB8: u128 = ONES8 << 3;
pub const WEIGHT_MSB16: u128 = ONES16 << 4;
pub const WEIGHT_MSB32: u128 = ONES32 << 5;
pub const WEIGHT_MSB64: u128 = ONES64 << 6;

impl Bits1<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits1::from_element(1), Bits1(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF));
    /// assert_eq!(Bits1::from_element(0), Bits1(0x0000_0000_0000_0000_0000_0000_0000_0000));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n1 = e | e << 1;
        let n2 = n1 | n1 << 2;
        let n3 = n2 | n2 << 4;
        let n4 = n3 | n3 << 8;
        let n5 = n4 | n4 << 16;
        let n6 = n5 | n5 << 32;
        let n7 = n6 | n6 << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits2<u128> {
        let (left, right) = self.split();
        left + right
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// For a Bits1, this is the same as computing the hamming weight from the
    /// original number and is a simple XOR.
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub fn split(self) -> (Bits1x2<u128>, Bits1x2<u128>) {
        let Self(n) = self;
        (
            Bits1x2(Bits2((n & LEFT_MASKS[6]) >> 1)),
            Bits1x2(Bits2(n & RIGHT_MASKS[6])),
        )
    }

    /// Takes the left and right sides and spreads them out
    /// so that the bits in each element are spread out into twice
    /// the amount of space.
    ///
    /// ```
    /// use swar::*;
    ///
    /// let input = Bits1(0b1101 << 64 | 0b0101u128);
    /// let (left, right) = input.halve();
    /// assert_eq!(left, Bits2(0b0101_0001));
    /// assert_eq!(right, Bits2(0b0001_0001));
    /// ```
    #[inline]
    pub fn halve(self) -> (Bits2<u128>, Bits2<u128>) {
        let Self(n) = self;
        let left = (n & LEFT_MASKS[0]) >> 64;
        let left = (left & LEFT_MASKS[1]) << 32 | left & RIGHT_MASKS[1];
        let left = (left & LEFT_MASKS[2]) << 16 | left & RIGHT_MASKS[2];
        let left = (left & LEFT_MASKS[3]) << 8 | left & RIGHT_MASKS[3];
        let left = (left & LEFT_MASKS[4]) << 4 | left & RIGHT_MASKS[4];
        let left = (left & LEFT_MASKS[5]) << 2 | left & RIGHT_MASKS[5];
        let left = (left & LEFT_MASKS[6]) << 1 | left & RIGHT_MASKS[6];
        let right = n & RIGHT_MASKS[0];
        let right = (right & LEFT_MASKS[1]) << 32 | right & RIGHT_MASKS[1];
        let right = (right & LEFT_MASKS[2]) << 16 | right & RIGHT_MASKS[2];
        let right = (right & LEFT_MASKS[3]) << 8 | right & RIGHT_MASKS[3];
        let right = (right & LEFT_MASKS[4]) << 4 | right & RIGHT_MASKS[4];
        let right = (right & LEFT_MASKS[5]) << 2 | right & RIGHT_MASKS[5];
        let right = (right & LEFT_MASKS[6]) << 1 | right & RIGHT_MASKS[6];
        (Bits2(left), Bits2(right))
    }
}

impl BitAnd<u128> for Bits1<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits1<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Bits2<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits2::from_element(0b10), Bits2(0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n2 = e | e << 2;
        let n3 = n2 | n2 << 4;
        let n4 = n3 | n3 << 8;
        let n5 = n4 | n4 << 16;
        let n6 = n5 | n5 << 32;
        let n7 = n6 | n6 << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits4<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// ```
    /// use swar::*;
    ///
    /// // All combinations of inputs 0-2 (hamming weights)
    /// let a = Bits2(0b00_01_10_00_01_10_00_01_10u128);
    /// let b = Bits2(0b00_00_00_01_01_01_10_10_10u128);
    /// // Expected output weights
    /// let e = Bits2(0b00_01_10_01_00_01_10_01_00u128);
    ///
    /// assert_eq!(a.hwd(b), e, "got hamming distances {:b} expected {:b}", a.hwd(b).0, e.0);
    /// ```
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        // I worked out the Karnaugh map and got the following:
        // High:
        // |0|0|x|1|
        // |0|0|x|0|
        // |x|x|x|x|
        // |1|0|x|0|
        // Low:
        // |0|1|x|0|
        // |1|0|x|1|
        // |x|x|x|x|
        // |0|1|x|0|
        // I reduced these maps to the following computation.
        // high = B1 & !A1 & !A0 | A1 & !B1 & !B0
        // low = !A0 & B0 | A0 & !B0
        // Please send PRs if you can improve this.
        let Self(a) = self;
        let Self(b) = other;
        let low = RIGHT_MASKS[6] & (a ^ b);
        let high = LEFT_MASKS[6] & (b & !a & !a << 1 | a & !b & !b << 1);
        Self(low | high)
    }

    #[inline]
    pub fn split(self) -> (Bits2x4<u128>, Bits2x4<u128>) {
        let Self(n) = self;
        (
            Bits2x4(Bits4((n & LEFT_MASKS[5]) >> 2)),
            Bits2x4(Bits4(n & RIGHT_MASKS[5])),
        )
    }
}

impl BitAnd<u128> for Bits2<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits2<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Bits4<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits4::from_element(0b0110), Bits4(0x6666_6666_6666_6666_6666_6666_6666_6666));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n3 = e | e << 4;
        let n4 = n3 | n3 << 8;
        let n5 = n4 | n4 << 16;
        let n6 = n5 | n5 << 32;
        let n7 = n6 | n6 << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits8<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// ```
    /// use swar::*;
    ///
    /// for a in 0u128..=4 {
    ///     for b in 0u128..=4 {
    ///         let aa = Bits4(a | a << 4);
    ///         let bb = Bits4(b | b << 4);
    ///         let out = aa.hwd(bb);
    ///         let diff = (a as i128 - b as i128).abs() as u128;
    ///         let expected = Bits4(diff | diff << 4);
    ///         assert_eq!(out, expected, "got hamming distances {:08b} expected {:08b} ({:04b}, {:04b})", out.0, expected.0, a, b);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        // Compute ABC + !DEF.
        let m = a + (b ^ WEIGHT_MASK4);
        // Get the MSB of the weight.
        let high = m & WEIGHT_MSB4;
        // If the MSB is not set, we need to add 1 (because -n = ~n + 1).
        let offset = (high ^ WEIGHT_MSB4) >> 2;
        // If the MSB is set, we need to flip all the bits.
        let flips = high | high >> 1 | high >> 2;
        // The order we apply the offset and flips in is irrelevant because
        // only one of the operations will have an effect anyways. We need
        // to mask out the higher bit at the end because it shouldnt be set.
        Self(((m ^ flips) + offset) & WEIGHT_MASK4)
    }

    #[inline]
    pub fn split(self) -> (Bits4x8<u128>, Bits4x8<u128>) {
        let Self(n) = self;
        (
            Bits4x8(Bits8((n & LEFT_MASKS[4]) >> 4)),
            Bits4x8(Bits8(n & RIGHT_MASKS[4])),
        )
    }
}

impl BitAnd<u128> for Bits4<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits4<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Bits8<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits8::from_element(0xFE), Bits8(0xFEFE_FEFE_FEFE_FEFE_FEFE_FEFE_FEFE_FEFE));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n4 = e | e << 8;
        let n5 = n4 | n4 << 16;
        let n6 = n5 | n5 << 32;
        let n7 = n6 | n6 << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits16<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// ```
    /// use swar::*;
    ///
    /// let bits = 8;
    /// for a in 0u128..=bits as u128 {
    ///     for b in 0u128..=bits as u128 {
    ///         let aa = Bits8(a | a << bits);
    ///         let bb = Bits8(b | b << bits);
    ///         let out = aa.hwd(bb);
    ///         let diff = (a as i128 - b as i128).abs() as u128;
    ///         let expected = Bits8(diff | diff << bits);
    ///         assert_eq!(out, expected, "got hamming distances {:016b} expected {:016b} ({:08b}, {:08b})", out.0, expected.0, a, b);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        // Compute a + !b for each substring.
        let m = a + (b ^ WEIGHT_MASK8);
        // Get the MSB of the weight.
        let high = m & WEIGHT_MSB8;
        // If the MSB is not set, we need to add 1 (because -n = ~n + 1).
        let offset = (high ^ WEIGHT_MSB8) >> 3;
        // If the MSB is set, we need to flip all the bits.
        let flips = high | high >> 1;
        let flips = flips | flips >> 2;
        // The order we apply the offset and flips in is irrelevant because
        // only one of the operations will have an effect anyways. We need
        // to mask out the higher bit at the end because it shouldnt be set.
        Self(((m ^ flips) + offset) & WEIGHT_MASK8)
    }

    #[inline]
    pub fn split(self) -> (Bits8x16<u128>, Bits8x16<u128>) {
        let Self(n) = self;
        (
            Bits8x16(Bits16((n & LEFT_MASKS[3]) >> 8)),
            Bits8x16(Bits16(n & RIGHT_MASKS[3])),
        )
    }
}

impl BitAnd<u128> for Bits8<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits8<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Bits16<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits16::from_element(0xFEED), Bits16(0xFEED_FEED_FEED_FEED_FEED_FEED_FEED_FEED));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n5 = e | e << 16;
        let n6 = n5 | n5 << 32;
        let n7 = n6 | n6 << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits32<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// ```
    /// use swar::*;
    ///
    /// let bits = 16;
    /// for a in 0u128..=bits as u128 {
    ///     for b in 0u128..=bits as u128 {
    ///         let aa = Bits16(a | a << bits);
    ///         let bb = Bits16(b | b << bits);
    ///         let out = aa.hwd(bb);
    ///         let diff = (a as i128 - b as i128).abs() as u128;
    ///         let expected = Bits16(diff | diff << bits);
    ///         assert_eq!(out, expected);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        // Compute a + !b for each substring.
        let m = a + (b ^ WEIGHT_MASK16);
        // Get the MSB of the weight.
        let high = m & WEIGHT_MSB16;
        // If the MSB is not set, we need to add 1 (because -n = ~n + 1).
        let offset = (high ^ WEIGHT_MSB16) >> 4;
        // If the MSB is set, we need to flip all the bits.
        let flips = high | high >> 1;
        let flips = flips | flips >> 2 | high >> 4;
        // The order we apply the offset and flips in is irrelevant because
        // only one of the operations will have an effect anyways. We need
        // to mask out the higher bit at the end because it shouldnt be set.
        Self(((m ^ flips) + offset) & WEIGHT_MASK16)
    }

    #[inline]
    pub fn split(self) -> (Bits16x32<u128>, Bits16x32<u128>) {
        let Self(n) = self;
        (
            Bits16x32(Bits32((n & LEFT_MASKS[2]) >> 16)),
            Bits16x32(Bits32(n & RIGHT_MASKS[2])),
        )
    }
}

impl BitAnd<u128> for Bits16<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits16<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Bits32<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits32::from_element(0xFEED_FACE), Bits32(0xFEED_FACE_FEED_FACE_FEED_FACE_FEED_FACE));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n6 = e | e << 32;
        let n7 = n6 | n6 << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits64<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// ```
    /// use swar::*;
    ///
    /// let bits = 32;
    /// for a in 0u128..=bits as u128 {
    ///     for b in 0u128..=bits as u128 {
    ///         let aa = Bits32(a | a << bits);
    ///         let bb = Bits32(b | b << bits);
    ///         let out = aa.hwd(bb);
    ///         let diff = (a as i128 - b as i128).abs() as u128;
    ///         let expected = Bits32(diff | diff << bits);
    ///         assert_eq!(out, expected);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        // Compute a + !b for each substring.
        let m = a + (b ^ WEIGHT_MASK32);
        // Get the MSB of the weight.
        let high = m & WEIGHT_MSB32;
        // If the MSB is not set, we need to add 1 (because -n = ~n + 1).
        let offset = (high ^ WEIGHT_MSB32) >> 5;
        // If the MSB is set, we need to flip all the bits.
        let flips = high | high >> 1;
        let flips = flips | flips >> 2;
        let flips = flips | flips >> 2;
        // The order we apply the offset and flips in is irrelevant because
        // only one of the operations will have an effect anyways. We need
        // to mask out the higher bit at the end because it shouldnt be set.
        Self(((m ^ flips) + offset) & WEIGHT_MASK32)
    }

    #[inline]
    pub fn split(self) -> (Bits32x64<u128>, Bits32x64<u128>) {
        let Self(n) = self;
        (
            Bits32x64(Bits64((n & LEFT_MASKS[1]) >> 32)),
            Bits32x64(Bits64(n & RIGHT_MASKS[1])),
        )
    }
}

impl BitAnd<u128> for Bits32<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits32<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Bits64<u128> {
    /// Spread a single value out to each element. Must be able to fit.
    ///
    /// ```
    /// use swar::*;
    ///
    /// assert_eq!(Bits64::from_element(0xFEED_FACE_CAFE_BEEF), Bits64(0xFEED_FACE_CAFE_BEEF_FEED_FACE_CAFE_BEEF));
    /// ```
    #[inline]
    pub fn from_element(e: u128) -> Self {
        // We can do this in log2(bits) time by doubling the sequence.
        let n7 = e | e << 64;
        Self(n7)
    }

    #[inline]
    pub fn sum_weight2(self) -> Bits128<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
    ///
    /// ```
    /// use swar::*;
    ///
    /// let bits = 64;
    /// for a in 0u128..=bits as u128 {
    ///     for b in 0u128..=bits as u128 {
    ///         let aa = Bits64(a | a << bits);
    ///         let bb = Bits64(b | b << bits);
    ///         let out = aa.hwd(bb);
    ///         let diff = (a as i128 - b as i128).abs() as u128;
    ///         let expected = Bits64(diff | diff << bits);
    ///         assert_eq!(out, expected);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn hwd(self, other: Self) -> Self {
        let Self(a) = self;
        let Self(b) = other;
        // Compute a + !b for each substring.
        let m = a + (b ^ WEIGHT_MASK64);
        // Get the MSB of the weight.
        let high = m & WEIGHT_MSB64;
        // If the MSB is not set, we need to add 1 (because -n = ~n + 1).
        let offset = (high ^ WEIGHT_MSB64) >> 6;
        // If the MSB is set, we need to flip all the bits.
        let flips = high | high >> 1;
        let flips = flips | flips >> 2;
        let flips = flips | flips >> 3;
        // The order we apply the offset and flips in is irrelevant because
        // only one of the operations will have an effect anyways. We need
        // to mask out the higher bit at the end because it shouldnt be set.
        Self(((m ^ flips) + offset) & WEIGHT_MASK64)
    }

    #[inline]
    pub fn split(self) -> (Bits64x128<u128>, Bits64x128<u128>) {
        let Self(n) = self;
        (
            Bits64x128(Bits128((n & LEFT_MASKS[0]) >> 64)),
            Bits64x128(Bits128(n & RIGHT_MASKS[0])),
        )
    }
}

impl BitAnd<u128> for Bits64<u128> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits64<u128> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl From<Bits128<u128>> for u128 {
    #[inline]
    fn from(n: Bits128<u128>) -> u128 {
        n.0
    }
}
