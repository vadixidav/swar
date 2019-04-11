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
//! ```

mod u128;

/// This is used when each bit is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits1<N>(pub N);

/// This is used when every `2` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits2<N>(pub N);

/// This is used when every `4` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits4<N>(pub N);

/// This is used when every `8` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits8<N>(pub N);

/// This is used when every `16` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits16<N>(pub N);

/// This is used when every `32` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits32<N>(pub N);

/// This is used when every `64` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits64<N>(pub N);

/// This is used when every `128` bits is a number stored in parallel.
#[derive(Copy, Clone, Debug)]
pub struct Bits128<N>(pub N);
