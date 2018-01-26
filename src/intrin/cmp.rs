// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

pub trait PackedCmp {
    /// Return a vector where each element at an index i is the maximum of the
    /// elements at index i in `self` and `other`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s(0).max(i8s(2)), i8s(2));
    /// assert_eq!(i8s::halfs(1, 0).max(i8s::halfs(2, -1)), i8s::halfs(2, 0));
    /// # }
    /// ```
    fn max(&self, other: Self) -> Self;

    /// Return a vector where each element at an index i is the minimum of the
    /// elements at index i in `self` and `other`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s(0).min(i8s(2)), i8s(0));
    /// assert_eq!(i8s::halfs(1, 0).min(i8s::halfs(2, -1)), i8s::halfs(1, -1));
    /// # }
    /// ```
    fn min(&self, other: Self) -> Self;

    /// Return a vector where each element at an index i is filled with 1s if the elements
    /// of `self` and `other` at index i are equal, and filled with zeroes otherwise.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s::interleave(0, 2).eq(u8s(0)), u8s::interleave(0xFF, 0));
    /// assert_eq!(u32s::halfs(1, 0).min(u32s(0)), u32s::halfs(0, 0xFFFFFFFF));
    /// # }
    /// ```
    fn eq(&self, other: Self) -> Self;
}

rust_fallback_impl_binary! {
    impl PackedCmp for u8x16 where "sse2" {
        min => _mm_min_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => _mm_max_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        eq => _mm_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for i8x16 where "sse4.1" {
        min => _mm_min_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => _mm_max_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        eq => _mm_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for u16x8 where "sse4.1" {
        min => _mm_min_epu16(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => _mm_max_epu16(), [0, 1, 2, 3, 4, 5, 6, 7];
        eq => _mm_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for i16x8 where "sse4.1" {
        min => _mm_min_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => _mm_max_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
        eq => _mm_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for u32x4 where "sse4.1" {
        min => _mm_min_epu32(), [0, 1, 2, 3];
        max => _mm_max_epu32(), [0, 1, 2, 3];
        eq => _mm_cmpeq_epi32(), [0, 1, 2, 3];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for i32x4 where "sse4.1" {
        min => _mm_min_epi32(), [0, 1, 2, 3];
        max => _mm_max_epi32(), [0, 1, 2, 3];
        eq => _mm_cmpeq_epi32(), [0, 1, 2, 3];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for f32x4 where "sse" {
        min => _mm_min_ps(), [0, 1, 2, 3];
        max => _mm_max_ps(), [0, 1, 2, 3];
        eq => _mm_cmpeq_ps(), [0, 1, 2, 3];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for f64x2 where "sse2" {
        min => _mm_min_pd(), [0, 1];
        max => _mm_max_pd(), [0, 1];
        eq => _mm_cmpeq_pd(), [0, 1];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for u8x32 where "avx2" {
        min => _mm256_min_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        max => _mm256_max_epu8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        eq => _mm256_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                     17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for i8x32 where "avx2" {
        min => _mm256_min_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        max => _mm256_max_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        eq => _mm256_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                     17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for u16x16 where "avx2" {
        min => _mm256_min_epu16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => _mm256_max_epu16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        eq => _mm256_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}


rust_fallback_impl_binary! {
    impl PackedCmp for i16x16 where "avx2" {
        min => _mm256_min_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        max => _mm256_max_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        eq => _mm256_cmpeq_epi16(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for u32x8 where "avx" {
        min => _mm256_min_epu32(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => _mm256_max_epu32(), [0, 1, 2, 3, 4, 5, 6, 7];
        eq => _mm256_cmpeq_epi32(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for i32x8 where "avx" {
        min => _mm256_min_epi32(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => _mm256_max_epi32(), [0, 1, 2, 3, 4, 5, 6, 7];
        eq => _mm256_cmpeq_epi32(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for f32x8 where "avx" {
        min => _mm256_min_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        max => _mm256_max_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
        eq => _mm256_cmp_ps(0x00), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl_binary! {
    impl PackedCmp for f64x4 where "avx" {
        min => _mm256_min_pd(), [0, 1, 2, 3];
        max => _mm256_max_pd(), [0, 1, 2, 3];
        eq => _mm256_cmp_pd(0x00), [0, 1, 2, 3];
    }
}
