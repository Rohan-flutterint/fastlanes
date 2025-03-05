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
    use std::hint::black_box;

    // const BENCH_W: [usize; 3] = [8, 1024, 1024 * 128, 1024 * 1024];
    const BENCH_W: [usize; 3] = [1, 4, 8];
    const BENCH_ARGS: &[(usize, usize)] = &[(2, 1), (3, 1), (5, 1), (2, 8), (3, 8), (5, 8)];

    #[divan::bench(types=[u8, u16, u32, u64], args = BENCH_ARGS)]
    fn bitpacking_cmp_fused<T>(bencher: Bencher, (w, len): (usize, usize))
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
    {
        let mut rng = StdRng::seed_from_u64(0);

        let mut values = vec![T::from_usize(2).expect(""); 1024 * len];
        for i in values.iter_mut() {
            *i = T::from_usize(rng.random_range(0..1 << w)).unwrap();
        }
        let mut packed = vec![T::zero(); len * 128 * w / size_of::<T>()];

        let pack_size: usize = 128 * w / size_of::<T>();

        for i in 0..len {
            unsafe {
                BitPacking::unchecked_pack(
                    w,
                    &values[i..][..1024],
                    &mut packed[i * pack_size..][..pack_size],
                )
            };
        }

        bencher
            .with_inputs(|| {
                let value = T::from_usize(1).expect("");
                let output = (0..len * 1024).map(|_| false).collect::<Vec<_>>();
                (packed.as_slice(), value, output)
            })
            .bench_local_refs(|x| {
                let (packed, value, output) = x;
                compare_fused(len, pack_size, w, packed, *value, output);
                black_box(output);
            });
    }

    #[inline(never)]
    fn compare_fused<T: PartialOrd + BitPackingCompare>(
        len: usize,
        packed_size: usize,
        w: usize,
        packed: &[T],
        value: T,
        output: &mut [bool],
    ) where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
    {
        unsafe {
            for i in 0..len {
                let unpacked = &mut output[i * 1024..][..1024];
                let packed_slice = &packed[i * packed_size..][..packed_size];
                BitPackingCompare::unchecked_unpack_cmp(
                    w,
                    packed_slice,
                    &mut *(unpacked.as_mut_ptr().cast()),
                    |a, b| a < b,
                    value,
                );
            }
        };
        black_box(output);
    }

    #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    fn bitpacking_cmp_seq_w3<T, const ELEM_SIZE: usize>(bencher: Bencher)
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
    {
        let mut rng = StdRng::seed_from_u64(0);

        const W: usize = 3;
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
                let value = T::from_usize(1).expect("");
                let output = (0..ELEM_SIZE * 1024).map(|_| false).collect::<Vec<_>>();
                (packed.as_slice(), value, output)
            })
            .bench_local_refs(|x| {
                let (packed, value, output) = x;
                compare_unfused(
                    ELEM_SIZE,
                    pack_size,
                    W,
                    packed,
                    &mut unpacked,
                    *value,
                    output,
                );
                black_box(output);
            });
    }

    #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    fn bitpacking_cmp_seq_w2<T, const ELEM_SIZE: usize>(bencher: Bencher)
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
    {
        let mut rng = StdRng::seed_from_u64(0);

        const W: usize = 2;
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
                let value = T::from_usize(1).expect("");
                let output = (0..ELEM_SIZE * 1024).map(|_| false).collect::<Vec<_>>();
                (packed.as_slice(), value, output)
            })
            .bench_local_refs(|x| {
                let (packed, value, output) = x;
                compare_unfused(
                    ELEM_SIZE,
                    pack_size,
                    W,
                    packed,
                    &mut unpacked,
                    *value,
                    output,
                );
                black_box(output);
            });
    }

    #[inline(never)]
    fn compare_unfused<T: PartialOrd + BitPackingCompare>(
        len: usize,
        packed_size: usize,
        w: usize,
        packed: &[T],
        unpacked: &mut [T],
        value: T,
        output: &mut [bool],
    ) where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
    {
        for i in 0..len {
            let packed_slice = &packed[i * packed_size..][..packed_size];
            unsafe { T::unchecked_unpack(w, &packed_slice, unpacked) };
            for j in 0..1024 {
                output[i * 1024 + j] = unpacked[j] < value;
            }
        }
        black_box(output);
    }

    // #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    // fn bitpacking_cmp_raw_w3<T, const NN: usize>(bencher: Bencher)
    // where
    //     T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
    //     T: BitPacking + BitPackingCompare + Copy,
    //     [(); 1024 * NN]:,
    // {
    //     const W: usize = 3;
    //     let mut rng = StdRng::seed_from_u64(0);
    //     let mut values = vec![T::from_usize(2).expect(""); 1024 * NN];
    //
    //     for i in values.iter_mut() {
    //         *i = T::from_usize(rng.random_range(0..1 << (W))).unwrap();
    //     }
    //
    //     bencher
    //         .with_inputs(|| {
    //             let value = T::from_usize(rng.random_range(0..1 << W)).expect("");
    //             let output = (0..NN * 1024).map(|_| false).collect::<Vec<_>>();
    //             (values.as_slice(), value, output)
    //         })
    //         .bench_local_refs(|x| {
    //             let (values, value, output) = x;
    //             compare_raw(values, *value, output);
    //             black_box(output);
    //         });
    // }
    //
    // #[divan::bench(types=[u8, u16, u32, u64], consts = BENCH_W)]
    // fn bitpacking_cmp_raw_w2<T, const NN: usize>(bencher: Bencher)
    // where
    //     T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
    //     T: BitPacking + BitPackingCompare + Copy,
    //     [(); 1024 * NN]:,
    // {
    //     const W: usize = 2;
    //     let mut rng = StdRng::seed_from_u64(0);
    //     let mut values = vec![T::from_usize(2).expect(""); 1024 * NN];
    //
    //     for i in values.iter_mut() {
    //         *i = T::from_usize(rng.random_range(0..1 << (W))).unwrap();
    //     }
    //
    //     bencher
    //         .with_inputs(|| {
    //             let value = T::from_usize(rng.random_range(0..1 << W)).expect("");
    //             let output = (0..NN * 1024).map(|_| false).collect::<Vec<_>>();
    //             (values.as_slice(), value, output)
    //         })
    //         .bench_local_refs(|x| {
    //             let (values, value, output) = x;
    //             compare_raw(values, *value, output);
    //             black_box(output);
    //         });
    // }

    #[divan::bench(types=[u8, u16, u32, u64], args = BENCH_ARGS)]
    fn bitpacking_cmp_raw<T>(bencher: Bencher, (w, len): (usize, usize))
    where
        T: BitPacking + FastLanesComparable<Bitpacked = T> + FromPrimitive + Copy,
        T: BitPacking + BitPackingCompare + Copy,
    {
        let mut rng = StdRng::seed_from_u64(0);

        let mut values = vec![T::from_usize(2).expect(""); 1024 * len];
        for i in values.iter_mut() {
            *i = T::from_usize(rng.random_range(0..1 << w)).unwrap();
        }

        bencher
            .with_inputs(|| {
                let value = T::from_usize(1).expect("");
                let output = (0..len * 1024).map(|_| false).collect::<Vec<_>>();
                (values.as_slice(), value, output)
            })
            .bench_local_refs(|x| {
                let (values, value, output) = x;
                compare_raw(values, *value, output);
                black_box(output);
            });
    }

    #[inline(never)]
    fn compare_raw<T: PartialOrd>(values: &[T], value: T, output: &mut [bool]) {
        for i in 0..values.len() {
            output[i] = values[i] < value;
        }
        black_box(output);
    }
}
