//! ## Counting bits iteratively
//! ```
//! use swar::*;
//! let n = 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAAu128;
//! let weight: u128 =
//!     Bits1(n).sum_weight2()
//!             .sum_weight2()
//!             .sum_weight2()
//!             .sum_weight2()
//!             .sum_weight2()
//!             .sum_weight2()
//!             .sum_weight2()
//!             .into();
//! assert_eq!(weight as u32, n.count_ones());
//!
//! assert_eq!(Bits1(n).sum_weight2().sum_weight2().0,
//!            0x2222_2222_2222_2222_2222_2222_2222_2222);
//!
//! assert_eq!(Bits1(n).sum_weight2().sum_weight2().split().0,
//!            Bits4x8(Bits8(0x0202_0202_0202_0202_0202_0202_0202_0202)));
//! ```
#![no_std]

pub mod u128;

/// This is used when each bit is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits1<N>(pub N);

/// This is used when every `2` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits2<N>(pub N);

/// This is used when every `4` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits4<N>(pub N);

/// This is used when every `8` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits8<N>(pub N);

/// This is used when every `16` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits16<N>(pub N);

/// This is used when every `32` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits32<N>(pub N);

/// This is used when every `64` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits64<N>(pub N);

/// This is used when every `128` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits128<N>(pub N);

/// This is used when each bit is a number stored in parallel with a stride of `2`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits1x2<N>(pub Bits2<N>);

/// This is used when every `2` bits is a number stored in parallel with a stride of `4`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits2x4<N>(pub Bits4<N>);

/// This is used when every `3` bits is a number stored in parallel with a stride of `4`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits3x4<N>(pub Bits4<N>);

/// This is used when every `4` bits is a number stored in parallel with a stride of `8`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits4x8<N>(pub Bits8<N>);

/// This is used when every `5` bits is a number stored in parallel with a stride of `8`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits5x8<N>(pub Bits8<N>);

/// This is used when every `8` bits is a number stored in parallel with a stride of `16`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits8x16<N>(pub Bits16<N>);

/// This is used when every `9` bits is a number stored in parallel with a stride of `16`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits9x16<N>(pub Bits16<N>);

/// This is used when every `16` bits is a number stored in parallel with a stride of `32`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits16x32<N>(pub Bits32<N>);

/// This is used when every `17` bits is a number stored in parallel with a stride of `32`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits17x32<N>(pub Bits32<N>);

/// This is used when every `32` bits is a number stored in parallel with a stride of `64`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits32x64<N>(pub Bits64<N>);

/// This is used when every `33` bits is a number stored in parallel with a stride of `64`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits33x64<N>(pub Bits64<N>);

/// This is used when every `64` bits is a number stored in parallel with a stride of `128`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits64x128<N>(pub Bits128<N>);

/// This is used when every `65` bits is a number stored in parallel with a stride of `128`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits65x128<N>(pub Bits128<N>);

impl<N> From<Bits1x2<N>> for Bits2<N> {
    fn from(n: Bits1x2<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits2x4<N>> for Bits4<N> {
    fn from(n: Bits2x4<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits3x4<N>> for Bits4<N> {
    fn from(n: Bits3x4<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits4x8<N>> for Bits8<N> {
    fn from(n: Bits4x8<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits5x8<N>> for Bits8<N> {
    fn from(n: Bits5x8<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits8x16<N>> for Bits16<N> {
    fn from(n: Bits8x16<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits9x16<N>> for Bits16<N> {
    fn from(n: Bits9x16<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits16x32<N>> for Bits32<N> {
    fn from(n: Bits16x32<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits17x32<N>> for Bits32<N> {
    fn from(n: Bits17x32<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits32x64<N>> for Bits64<N> {
    fn from(n: Bits32x64<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits33x64<N>> for Bits64<N> {
    fn from(n: Bits33x64<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits64x128<N>> for Bits128<N> {
    fn from(n: Bits64x128<N>) -> Self {
        n.0
    }
}

impl<N> From<Bits65x128<N>> for Bits128<N> {
    fn from(n: Bits65x128<N>) -> Self {
        n.0
    }
}
