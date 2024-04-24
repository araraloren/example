#![feature(test)]

use std::hint::black_box;

use test::Bencher;

extern crate test;

#[bench]
fn bech_tiny_array(b: &mut Bencher) {
    b.iter(|| array::try_array::<2>());
}

#[bench]
fn bech_tiny_array2(b: &mut Bencher) {
    b.iter(|| array::try_array::<6>());
}

#[bench]
fn bech_tiny_array3(b: &mut Bencher) {
    b.iter(|| array::try_array::<8>());
}

#[bench]
fn bech_large_array(b: &mut Bencher) {
    b.iter(|| array::try_array::<128>());
}

#[bench]
fn bech_large_array2(b: &mut Bencher) {
    b.iter(|| array::try_array::<256>());
}

#[bench]
fn bech_large_array3(b: &mut Bencher) {
    b.iter(|| array::try_array::<512>());
}

#[bench]
fn bech_large_array4(b: &mut Bencher) {
    b.iter(|| array::try_array::<1024>());
}

#[bench]
fn bech_large_array5(b: &mut Bencher) {
    b.iter(|| array::try_array::<2048>());
}

#[bench]
fn bech_tiny_vector(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<2>());
}

#[bench]
fn bech_tiny_vector2(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<6>());
}

#[bench]
fn bech_tiny_vector3(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<8>());
}

#[bench]
fn bech_large_vector(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<128>());
}

#[bench]
fn bech_large_vector2(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<256>());
}

#[bench]
fn bech_large_vector3(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<512>());
}

#[bench]
fn bech_large_vector4(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<1024>());
}

#[bench]
fn bech_large_vector5(b: &mut Bencher) {
    b.iter(|| vector::try_vector::<2048>());
}

mod array {
    use std::hint::black_box;

    pub fn try_array<const N: usize>() {
        let a: [u64; N] = construct_array();
        let mut sum = a.iter().sum::<u64>();

        black_box(sum);
    }

    fn construct_array<const N: usize, T: From<u64> + Copy>() -> [T; N] {
        let mut ret = [T::from(0); N];

        for (i, val) in ret.iter_mut().enumerate() {
            *val = T::from(i as u64);
        }
        ret
    }
}

mod vector {
    use std::hint::black_box;

    pub fn try_vector<const N: usize>() {
        let a = construct_vector::<N, u64>();
        let mut sum = a.iter().sum::<u64>();

        black_box(sum);
    }

    fn construct_vector<const N: usize, T: From<u64> + Copy>() -> Vec<T> {
        let mut ret = vec![T::from(0); N];

        for (i, val) in ret.iter_mut().enumerate() {
            *val = T::from(i as u64);
        }
        ret
    }
}
