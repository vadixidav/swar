# swar

SIMD Within a Register routine support crate

See <https://en.wikipedia.org/wiki/SWAR> for more information.

Also check [the documentation](https://docs.rs/swar/) to see actual code examples.

Note that this crate will be completely refactored after const generics lands in nightly. Future versions of this crate will then depend on nightly until const generics is stable.

This crate attempts to collect SIMD Within a Register (SWAR) routines and present them in a type-safe manner so that the programmer can't make a mistake with the layout of the number. A SWAR routine takes advantage of the fact that some operations can be computed in parallel in a single register without any SIMD extentions to the host processor at all. For instance, one common use of SWAR is in the rust `count_ones` instruction for targets without a dedicated `popcnt` instruction.

It is possible to mask every other bit in the number:

`X = ABCD & 1010 = A0C0`
`Y = ABCD & 0101 = 0B0D`

We can then shift the number by 1 and sum the bits:

`S1 = (X >> 1) + Y = EEFF`

The `EE` and `FF` noted above are now the hamming weights of the corresponding 2 bits `00`, `01`, or `10`.

We can now add these together as well:

`S2 = ((S1 & 1100) >> 2) + (S1 & 0011) = 0GGG`

Now the three bits `GGG` can vary in the range `0..=4` because the sum of bits (hamming weight) can be as large as four.

This pattern can continue and you can compute the hamming weight/`count_ones`/`popcnt` for `N` bits in `O(log2(N))` time.

It is also important to be able to compute the hamming distance between two binary feature vectors. We can represent a binary feature vector compactly in memory by assigning each bit to be a feature. Then, to compute the hamming distance, one only needs to use the `XOR` operation. Since `XOR` sets each bit which was different in the original input registers, the output register has the bits set which were different between them. Now, this output can have its hamming weight calculated, and this computes the hamming distance!

Due to modern systems pipelining, this can actually execute almost as fast as the native `popcnt` instruction in actual workloads, but huge disclaimers on that because the native instruction is definitely faster than the parallel bitcount solution on modern processors in simple benchmarks (see [here](http://0x80.pl/articles/sse-popcount.html)) and the difference in routine size can also affect cache pressure, so don't forget to benchmark! Obviously, use `popcnt` if it is available! If it isn't, there is still a good fallback available. Also consider using one of the SIMD variants in the reference that use the parallel bitcount, but use SIMD to push it to compute on even more bits.

The main reason you might wish to use this version of parallel bit counting is if you actually need the intermediary bit counts! It turns out this is actually important for the implementation of the [`hwt` crate](https://github.com/vadixidav/hwt), which is what inspired me to make this library.

We can also do other things in parallel. For instance, we can add in a register by adding single padding zeros to store the carry bits (adding two `n` bit numbers makes one `n+1` bit number):

- `OUT = 0A0B + 0C0D`
- `OUT & 0011 = B + D`
- `OUT & 1100 = (A + C) << 2`

We can subtract in a register by padding with `1` borrow bits:

- `OUT = 1A1B - 1C1D`
- `OUT & 0011 = B - D + 10`
- `OUT & 1100 = (A - C + 10) << 2`

We can even multiply in parallel within a register:

- `OUT = 0AB000CD * 0EF000GH`
- `OUT & 0000_0000_0000_1111 = CD * GH`
- `OUT & 0000_0011_1110_0000 = (AB * GH + CD * EF) << 5`
- `OUT & 0011_1100_0000_0000 = (AB * EF) << 10`

Note that we actually get three outputs from this multiplication! Multiplication isn't significantly more
efficient because we cannot pack the numbers very well, but it is possible in some scenarios.

I don't know if SWAR division is really possible using any methods I know, but please feel free to open an issue or PR if you have another SWAR algorithm for division!

## Credits

The [Stanford Bit Twiddling Hacks](https://graphics.stanford.edu/~seander/bithacks.html) page by Sean Eron Anderson is the source of some and inspiration for other bit twiddling algorithms found in this code.