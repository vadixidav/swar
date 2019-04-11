use crate::*;

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
        let Self(n) = self;
        Bits2(((n & LEFT_MASKS[6]) >> 1) + (n & RIGHT_MASKS[6]))
    }
}

impl Bits2<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits4<u128> {
        let Self(n) = self;
        Bits4(((n & LEFT_MASKS[5]) >> 2) + (n & RIGHT_MASKS[5]))
    }
}

impl Bits4<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits8<u128> {
        let Self(n) = self;
        Bits8(((n & LEFT_MASKS[4]) >> 4) + (n & RIGHT_MASKS[4]))
    }
}

impl Bits8<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits16<u128> {
        let Self(n) = self;
        Bits16(((n & LEFT_MASKS[3]) >> 8) + (n & RIGHT_MASKS[3]))
    }
}

impl Bits16<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits32<u128> {
        let Self(n) = self;
        Bits32(((n & LEFT_MASKS[2]) >> 16) + (n & RIGHT_MASKS[2]))
    }
}

impl Bits32<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits64<u128> {
        let Self(n) = self;
        Bits64(((n & LEFT_MASKS[1]) >> 32) + (n & RIGHT_MASKS[1]))
    }
}

impl Bits64<u128> {
    #[inline]
    pub fn sum_weight2(self) -> Bits128<u128> {
        let Self(n) = self;
        Bits128(((n & LEFT_MASKS[0]) >> 64) + (n & RIGHT_MASKS[0]))
    }
}

impl From<Bits128<u128>> for u128 {
    fn from(bits: Bits128<u128>) -> u128 {
        // There are 128 bits in a 128-bit number, therefore this is an identity.
        bits.0
    }
}
