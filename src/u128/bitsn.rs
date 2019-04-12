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
pub const WEIGHT_MSB4: u128 = ONES2 << 2;
pub const WEIGHT_MSB8: u128 = ONES2 << 3;
pub const WEIGHT_MSB16: u128 = ONES2 << 4;
pub const WEIGHT_MSB32: u128 = ONES2 << 5;
pub const WEIGHT_MSB64: u128 = ONES2 << 6;

impl Bits1<u128> {
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
    #[inline]
    pub fn sum_weight2(self) -> Bits8<u128> {
        let (left, right) = self.split();
        (left + right).into()
    }

    /// This computes the hamming weight distance from hamming weights.
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
        // If the MSB is set, we need to flip all the bits, but not add 1.
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
    #[inline]
    pub fn sum_weight2(self) -> Bits16<u128> {
        let (left, right) = self.split();
        (left + right).into()
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
    #[inline]
    pub fn sum_weight2(self) -> Bits32<u128> {
        let (left, right) = self.split();
        (left + right).into()
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
    #[inline]
    pub fn sum_weight2(self) -> Bits64<u128> {
        let (left, right) = self.split();
        (left + right).into()
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
    #[inline]
    pub fn sum_weight2(self) -> Bits128<u128> {
        let (left, right) = self.split();
        (left + right).into()
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
