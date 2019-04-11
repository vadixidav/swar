use crate::*;
use core::ops::Add;

impl Add for Bits1x2<u128> {
    type Output = Bits2<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits2((self.0).0 + (other.0).0)
    }
}

impl Add for Bits2x4<u128> {
    type Output = Bits3x4<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits3x4(Bits4((self.0).0 + (other.0).0))
    }
}

impl Add for Bits4x8<u128> {
    type Output = Bits5x8<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits5x8(Bits8((self.0).0 + (other.0).0))
    }
}

impl Add for Bits8x16<u128> {
    type Output = Bits9x16<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits9x16(Bits16((self.0).0 + (other.0).0))
    }
}

impl Add for Bits16x32<u128> {
    type Output = Bits17x32<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits17x32(Bits32((self.0).0 + (other.0).0))
    }
}

impl Add for Bits32x64<u128> {
    type Output = Bits33x64<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits33x64(Bits64((self.0).0 + (other.0).0))
    }
}

impl Add for Bits64x128<u128> {
    type Output = Bits65x128<u128>;

    fn add(self, other: Self) -> Self::Output {
        Bits65x128(Bits128((self.0).0 + (other.0).0))
    }
}
