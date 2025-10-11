#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as Map;

#[cfg(feature = "std")]
use std::collections::HashMap as Map;


#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;


#[cfg(not(feature = "std"))]
use alloc::vec;

#[cfg(feature = "std")]
use std::vec;

pub mod font;

pub mod render;
pub mod tables;
pub mod rasterizer;
pub mod cache;

pub use crate::font::TrueTypeFont;


pub trait F32NoStd {
    fn floor(self) -> f32;
    fn ceil(self) -> f32;
    fn round(self) -> f32;
    fn abs(self) -> f32;
}

impl F32NoStd for f32 {
    #[inline]
    fn floor(self) -> f32 {
        let xi = self as i32;
        if self < xi as f32 {
            xi as f32 - 1.0
        } else {
            xi as f32
        }
    }

    #[inline]
    fn ceil(self) -> f32 {
        let xi = self as i32;
        if self > xi as f32 {
            xi as f32 + 1.0
        } else {
            xi as f32
        }
    }

    #[inline]
    fn round(self) -> f32 {
        if self >= 0.0 {
            (self + 0.5).floor()
        } else {
            (self - 0.5).ceil()
        }
    }
    
    #[inline]
    fn abs(self) -> f32 {
        if self < 0.0 { -self } else { self }
    }
}


