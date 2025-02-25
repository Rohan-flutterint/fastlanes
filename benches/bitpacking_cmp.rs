#![feature(generic_const_exprs)]
#[allow(incomplete_features)]
use divan::Bencher;
use fastlanes::{BitPackWidth, BitPacking, FastLanes, SupportedBitPackWidth};
use num_traits::{FromPrimitive, Num};
use std::hint::black_box;

fn main() {
    divan::main();
}

// #[divan::bench(types=[u8, u32, u64], consts=[7])]
// fn bitpacking_cmp<T>(bencher: Bencher)
// where
//     T: Num + Copy + FromPrimitive + BitPacking + FastLanes,
//     BitPackWidth<W>: SupportedBitPackWidth<T>,
//     [(); 1024 * W / T::T]:,
// {
//     const W: usize = 7;
//     let values = [T::zero(); 1024];
//     let mut packed = [0; 1024 * W / T::T];
//     BitPacking::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 1024 / 64];
//     bencher.bench(|| {
//         black_box(BitPacking::unpack_eq::<W>(
//             &packed,
//             &mut unpacked,
//             T::from_usize(1).unwrap(),
//         ))
//     });
// }

// #[divan::bench(types=[u8, u32, u64], consts=[7])]
// fn bitpacking_unpack_cmp<T, const W: usize>(bencher: Bencher)
// where
//     T: Num + Copy + FromPrimitive + BitPacking,
//     BitPackWidth<W>: SupportedBitPackWidth<T>,
// {
//     let values = [T::zero(); 1024];
//     let mut packed = [0u64; 1024 * W / size_of::<T>()];
//     BitPacking::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [T::zero(); 1024];
//     bencher.bench(|| {
//         black_box(BitPacking::unpack::<W>(&packed, &mut unpacked));
//         black_box(collect_bool(unpacked.len(), |idx| {
//             unpacked[idx] == T::from_usize(1).unwrap()
//         }))
//     })
// }

#[inline]
pub fn ceil(value: usize, divisor: usize) -> usize {
    // Rewrite as `value.div_ceil(&divisor)` after
    // https://github.com/rust-lang/rust/issues/88581 is merged.
    value / divisor + (0 != value % divisor) as usize
}

#[inline]
pub fn collect_bool<F: FnMut(usize) -> bool>(len: usize, mut f: F) -> Vec<u64> {
    let mut buffer = Vec::with_capacity(ceil(len, 64) * 8);

    let chunks = len / 64;
    let remainder = len % 64;
    for chunk in 0..chunks {
        let mut packed = 0;
        for bit_idx in 0..64 {
            let i = bit_idx + chunk * 64;
            packed |= (f(i) as u64) << bit_idx;
        }

        // SAFETY: Already allocated sufficient capacity
        buffer.push(packed)
    }

    if remainder != 0 {
        let mut packed = 0;
        for bit_idx in 0..remainder {
            let i = bit_idx + chunks * 64;
            packed |= (f(i) as u64) << bit_idx;
        }

        // SAFETY: Already allocated sufficient capacity
        buffer.push(packed)
    }

    buffer.truncate(ceil(len, 8));
    buffer
}

//     {
//         let mut group = c.benchmark_group("unpack_eq");
//         group.bench_function("16 <- 3 stack", |b| {
//             const WIDTH: usize = 7;
//             let values = [4u64; 1024];
//             let mut packed = [0; 128 * WIDTH / size_of::<u64>()];
//             BitPacking::pack::<WIDTH>(&values, &mut packed);
//
//             let mut unpacked = [0u64; 1024 / 64];
//             b.iter(|| black_box(BitPacking::unpack_eq::<WIDTH>(&packed, &mut unpacked, 1)));
//         });
//     }
//
//     {
//         let mut group = c.benchmark_group("unpack_eq_coll");
//         group.bench_function("16 <- 3 stack", |b| {

//         });
//     }
