#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(unexpected_cfgs)]

fn main() {
    divan::main();
}

#[cfg(not(codspeed))]
mod bench {
    use divan::Bencher;
    use fastlanes::{BitPacking, BitPackingCompare, FastLanesComparable};
    use num_traits::FromPrimitive;
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};
    use std::fmt::{Debug, Display};

    // const BENCH_W: [usize; 3] = [8, 1024, 1024 * 128, 1024 * 1024];
    const BENCH_W: [usize; 5] = [1, 8, 16, 1024, 8 * 1024];

    #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    fn bitpacking_cmp_fused<T: Debug, const NN: usize>(bencher: Bencher)
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy + Display,
        [(); NN * 128 * 3 / size_of::<T>()]:,
        [(); 128 * 3 / size_of::<T>()]:,
        [(); 1024 * NN]:,
    {
        let mut rng = StdRng::seed_from_u64(0);

        const W: usize = 3;
        let value = T::from_usize(1).expect("");
        let mut values = vec![T::from_usize(2).expect(""); 1024 * NN];
        for i in values.iter_mut() {
            *i = T::from_usize(rng.random_range(0..1 << (W))).unwrap();
        }
        let mut packed = vec![T::zero(); NN * 128 * W / size_of::<T>()];

        let pack_size: usize = 128 * W / size_of::<T>();

        for i in 0..NN {
            unsafe {
                BitPacking::unchecked_pack(
                    W,
                    &values[i..][..1024],
                    &mut packed[i * pack_size..][..pack_size],
                )
            };
        }

        bencher
            .with_inputs(|| {
                let output = (0..NN * 1024).map(|_| false).collect::<Vec<_>>();
                (values.clone(), value, output)
            })
            .bench_local_values(|(packed, value, mut output)| {
                unsafe {
                    for i in 0..NN {
                        let unpacked = &mut output.as_mut_slice()[i * 1024..][..1024];
                        let packed_slice = &packed[i * pack_size..][..pack_size];
                        BitPackingCompare::unchecked_unpack_cmp(
                            W,
                            packed_slice,
                            &mut *(unpacked.as_mut_ptr().cast() as *mut [bool; 1024]),
                            |a, b| a < b,
                            value,
                        );
                    }
                };
                output
            });
    }

    #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    fn bitpacking_cmp_seq<T, const ELEM_SIZE: usize>(bencher: Bencher)
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
        [(); ELEM_SIZE * 128 * 3 / size_of::<T>()]:,
        [(); 1024 * ELEM_SIZE]:,
    {
        let mut rng = StdRng::seed_from_u64(0);

        const W: usize = 3;
        let value = T::from_usize(1).expect("");
        let mut values = vec![T::from_usize(2).expect(""); 1024 * ELEM_SIZE];
        let mut packed = vec![T::zero(); ELEM_SIZE * 128 * W / size_of::<T>()];
        for i in values.iter_mut() {
            *i = T::from_usize(rng.random_range(0..1 << (W))).unwrap();
        }

        let pack_size: usize = 128 * W / size_of::<T>();

        for i in 0..ELEM_SIZE {
            unsafe {
                BitPacking::unchecked_pack(
                    W,
                    &values[i..][..1024],
                    &mut packed[i..][..128 * W / size_of::<T>()],
                )
            };
        }

        let mut unpacked = [T::zero(); 1024];

        bencher
            .with_inputs(|| {
                let output = (0..ELEM_SIZE * 1024).map(|_| false).collect::<Vec<_>>();
                (values.clone(), value.clone(), output)
            })
            .bench_local_values(|(packed, value, mut output)| {
                for i in 0..ELEM_SIZE {
                    let packed_slice = &packed[i * pack_size..][..pack_size];
                    unsafe { T::unchecked_unpack(W, &packed_slice, &mut unpacked) };
                    for j in 0..1024 {
                        output[i * 1024 + j] = unpacked[j] < value;
                    }
                }
                output

                // unsafe {
                //     for i in 0..ELEM_SIZE {
                //         let packed_slice = &packed[i..][..1024];
                //         BitPackingCompare::unchecked_unpack_cmp(
                //             W,
                //             packed_slice,
                //             &mut unpacked,
                //             |a, b| a == b,
                //             black_box(value),
                //         );
                //         black_box(unpacked);
                //     }
                // };
            });
    }

    #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    fn bitpacking_cmp_raw<T, const NN: usize>(bencher: Bencher)
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
        [(); NN * 128 * 3 / size_of::<T>()]:,
        [(); 1024 * NN]:,
    {
        let mut rng = StdRng::seed_from_u64(0);

        const W: usize = 3;
        let value = T::from_usize(1).expect("");
        let mut values = vec![T::from_usize(2).expect(""); 1024 * NN];

        for i in values.iter_mut() {
            *i = T::from_usize(rng.random_range(0..1 << (W))).unwrap();
        }

        bencher
            .with_inputs(|| {
                let output = (0..NN * 1024).map(|_| false).collect::<Vec<_>>();
                (values.clone(), value.clone(), output)
            })
            .bench_local_values(|(values, value, mut output)| {
                for i in 0..NN * 1024 {
                    output[i] = values[i] < value;
                }
                output
            });
    }

    // #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    // fn bitpacking_cmp_seq<T, const W: usize>(bencher: Bencher)
    // where
    //     T: BitPacking + FromPrimitive + Copy,
    //     [(); 128 * W / size_of::<T>()]:,
    // {
    //     let value = T::from_usize(1).expect("");
    //     let values = [T::from_usize(2).expect(""); 1024 * 1024];
    //     let mut packed = [T::zero(); 128 * W / size_of::<T>()];
    //
    //     for i in 0..1024 {
    //         unsafe {
    //             BitPacking::unchecked_pack(
    //                 W,
    //                 &values[i..][..1024],
    //                 &mut packed[i..][..128 * W / size_of::<T>()],
    //             )
    //         };
    //     }
    //
    //     let mut unpacked = [T::zero(); 1024];
    //     let mut bools = [false; 1024];
    //
    //     bencher
    //         .with_inputs(|| (value, packed))
    //         .bench_local_refs(|(value, packed)| {
    //             for i in 0..1024 {
    //                 let packed_slice = &mut packed[i..][..1024];
    //                 unsafe { T::unchecked_unpack(W, &packed_slice, &mut unpacked) };
    //                 for i in 0..1024 {
    //                     bools[i] = unpacked[i] < *value
    //                 }
    //             }
    //         });
    // }
    // //
    // #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    // fn bitpacking_cmp_unpack<T, const W: usize>(bencher: Bencher)
    // where
    //     T: BitPacking + FromPrimitive + Copy,
    //     [(); 128 * W / size_of::<T>()]:,
    // {
    //     let values = [T::from_usize(2).expect(""); 1024];
    //     let mut packed = [T::zero(); 128 * W / size_of::<T>()];
    //
    //     unsafe { T::unchecked_pack(W, &values, &mut packed) };
    //
    //     let mut unpacked = [T::zero(); 1024];
    //
    //     bencher.bench_local(|| {
    //         unsafe { T::unchecked_unpack(W, &packed, &mut unpacked) };
    //     });
    // }
    //
    // pub fn collect_bool_cmp<T: PartialEq + Copy>(
    //     unpacked: &[T; 1024],
    //     cmp: &T,
    //     output: &mut [u64; 16],
    // ) {
    //     collect_bool(|idx| unpacked[idx] == *cmp, output);
    // }
    //
    // #[inline]
    // pub fn collect_bool<F: FnMut(usize) -> bool>(mut f: F, output: &mut [u64; 16]) {
    //     for chunk in 0..16 {
    //         let mut packed = 0;
    //         for bit_idx in 0..64 {
    //             let i = bit_idx + chunk * 64;
    //             packed |= u64::from(f(i)) << bit_idx;
    //         }
    //
    //         // SAFETY: Already allocated sufficient capacity
    //         output[chunk] = packed;
    //     }
    // }
}
