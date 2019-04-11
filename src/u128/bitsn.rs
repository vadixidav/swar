use crate::*;
use core::ops::BitAnd;
use core::ops::Shr;

const LEFT_MASKS: [u128; 7] = [
    0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0000,
    0xFFFF_FFFF_0000_0000_FFFF_FFFF_0000_0000,
    0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000,
    0xFF00_FF00_FF00_FF00_FF00_FF00_FF00_FF00,
    0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0,
    0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC,
    0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA,
];

const RIGHT_MASKS: [u128; 7] = [
    0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF,
    0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF,
    0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF,
    0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF,
    0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F,
    0x3333_3333_3333_3333_3333_3333_3333_3333,
    0x5555_5555_5555_5555_5555_5555_5555_5555,
];

impl Bits1<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits2<u128> {
        let (left, right) = self.split();
        left + right
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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits1<u128> {
    type Output = Self;

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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits2<u128> {
    type Output = Self;

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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits4<u128> {
    type Output = Self;

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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits8<u128> {
    type Output = Self;

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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits16<u128> {
    type Output = Self;

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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits32<u128> {
    type Output = Self;

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

    fn bitand(self, rhs: u128) -> Self {
        Self(self.0 & rhs)
    }
}

impl Shr<u32> for Bits64<u128> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl From<Bits128<u128>> for u128 {
    fn from(n: Bits128<u128>) -> u128 {
        n.0
    }
}