#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use divan::Bencher;
use fastlanes::test::collect_bool_cmp;
use fastlanes::{BitPacking, BitPackingCompare};
use std::hint::black_box;

fn main() {
    divan::main();
}

// #[divan::bench]
// fn bitpacking_cmp_u64_w3_fused(bencher: Bencher) {
//     const W: usize = 3;
//     type T = u64;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u64_w3_seq(bencher: Bencher) {
//     const W: usize = 3;
//     type T = u64;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u64_w15_fused(bencher: Bencher) {
//     const W: usize = 15;
//     type T = u64;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u64_w15_seq(bencher: Bencher) {
//     const W: usize = 15;
//     type T = u64;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u64_w16_fused(bencher: Bencher) {
//     const W: usize = 16;
//     type T = u64;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u64_w16_seq(bencher: Bencher) {
//     const W: usize = 16;
//     type T = u64;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
#[divan::bench]
fn bitpacking_cmp_u32_w3_fused(bencher: Bencher) {
    const W: usize = 3;
    type T = u32;
    let values = [2; 1024];
    let mut packed = [0; 128 * W / size_of::<T>()];
    T::pack::<W>(&values, &mut packed);

    let mut unpacked = [0u64; 16];
    bencher.bench_local(|| {
        black_box(T::unpack_cmp::<W, _>(
            &packed,
            &mut unpacked,
            |a, b| a == b,
            1,
        ));
    });
}

#[divan::bench]
fn bitpacking_cmp_u32_w3_seq(bencher: Bencher) {
    const W: usize = 3;
    type T = u32;
    let values = [2; 1024];
    let mut packed = [0; 128 * W / size_of::<T>()];
    T::pack::<W>(&values, &mut packed);

    let mut unpacked = [0; 1024];

    bencher.bench_local(|| {
        T::unpack::<W>(&packed, &mut unpacked);
        black_box(collect_bool_cmp(&unpacked, &1));
    });
}
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w16_fused(bencher: Bencher) {
//     const W: usize = 16;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w16_seq(bencher: Bencher) {
//     const W: usize = 16;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w17_fused(bencher: Bencher) {
//     const W: usize = 17;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w17_seq(bencher: Bencher) {
//     const W: usize = 17;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w18_fused(bencher: Bencher) {
//     const W: usize = 18;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w18_seq(bencher: Bencher) {
//     const W: usize = 18;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w19_fused(bencher: Bencher) {
//     const W: usize = 19;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u32_w19_seq(bencher: Bencher) {
//     const W: usize = 19;
//     type T = u32;
//     let values = [2; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//

// #[divan::bench]
// fn bitpacking_cmp_u16_w3_fused(bencher: Bencher) {
//     const W: usize = 3;
//     type T = u16;
//     let values = [3; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u16_w3_seq(bencher: Bencher) {
//     const W: usize = 3;
//     type T = u16;
//     let values = [3; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u8_w3_fused(bencher: Bencher) {
//     const W: usize = 3;
//     type T = u8;
//     let values = [3; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0u64; 16];
//     bencher.bench_local(|| {
//         black_box(T::unpack_cmp::<W, _>(
//             &packed,
//             &mut unpacked,
//             |a, b| a == b,
//             1,
//         ));
//     });
// }
//
// #[divan::bench]
// fn bitpacking_cmp_u8_w3_seq(bencher: Bencher) {
//     const W: usize = 3;
//     type T = u8;
//     let values = [3; 1024];
//     let mut packed = [0; 128 * W / size_of::<T>()];
//     T::pack::<W>(&values, &mut packed);
//
//     let mut unpacked = [0; 1024];
//
//     bencher.bench_local(|| {
//         T::unpack::<W>(&packed, &mut unpacked);
//         black_box(collect_bool_cmp(&unpacked, &1));
//     });
// }
