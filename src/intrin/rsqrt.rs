// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

// TODO: Guards and non-simd

pub trait Rsqrt {
    /// Return a vector containing an approximation of the reciprocals of the
    /// square-roots of elements in `self`. May contain significant float error
    /// past 10^-3.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert!(0.33333333 - 0.01 < f32s(9.0).rsqrt().coalesce() &&
    ///         0.33333333 + 0.01 > f32s(9.0).rsqrt().coalesce());
    /// # }
    /// ```
    fn rsqrt(&self) -> Self;
}

rust_fallback_impl! {
    impl Rsqrt for f32x8 where "avx" {
        rsqrt => _mm256_rsqrt_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl Rsqrt for f32x4 where "sse" {
        rsqrt => _mm_rsqrt_ps(), [0, 1, 2, 3];
    }
}

impl Rsqrt for f32 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        self.sqrt().recip()
    }
}

impl Rsqrt for f64 {
    #[inline(always)]
    fn rsqrt(&self) -> Self {
        self.sqrt().recip()
    }
}
