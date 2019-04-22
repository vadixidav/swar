//! ## Computing hamming weights in parallel
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
//!            Bits8(0x0202_0202_0202_0202_0202_0202_0202_0202));
//! ```
//!
//! ## Finding hamming weight differences in parallel
//! ```
//! use swar::*;
//!
//! // All combinations of inputs 0-2 (hamming weights)
//! let a = Bits2(0b00_01_10_00_01_10_00_01_10u128);
//! let b = Bits2(0b00_00_00_01_01_01_10_10_10u128);
//! // Expected output weights
//! let expected = Bits2(0b00_01_10_01_00_01_10_01_00u128);
//!
//! assert_eq!(a.minhwd(b), expected);
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
