//! `num_traits` implementations for UUID.
//!
//! This module provides comprehensive implementations of traits from the
//! [`num_traits`](https://docs.rs/num-traits) crate, enabling UUID to be used
//! in generic numeric contexts.
//!
//! # Implemented Traits
//!
//! ## Identity and Bounds
//! - [`Zero`], [`ConstZero`] - Additive identity (nil UUID)
//! - [`One`], [`ConstOne`] - Multiplicative identity
//! - [`Bounded`] - Minimum and maximum values
//!
//! ## Core Numeric
//! - [`Num`] - Core numeric trait with `from_str_radix`
//! - [`Unsigned`] - Marker trait for unsigned types
//! - [`PrimInt`] - Primitive integer operations
//!
//! ## Type Conversion
//! - [`ToPrimitive`] - Convert to primitive types
//! - [`FromPrimitive`] - Convert from primitive types
//! - [`NumCast`] - Generic numeric casting
//! - [`AsPrimitive`] - Unchecked casting (as semantics)
//!
//! ## Checked Operations
//! - [`CheckedAdd`], [`CheckedSub`], [`CheckedMul`], [`CheckedDiv`], [`CheckedRem`]
//! - [`CheckedNeg`], [`CheckedShl`], [`CheckedShr`]
//!
//! ## Saturating Operations
//! - [`Saturating`], [`SaturatingAdd`], [`SaturatingSub`], [`SaturatingMul`]
//!
//! ## Wrapping Operations
//! - [`WrappingAdd`], [`WrappingSub`], [`WrappingMul`]
//! - [`WrappingNeg`], [`WrappingShl`], [`WrappingShr`]
//!
//! ## Overflowing Operations
//! - [`OverflowingAdd`], [`OverflowingSub`], [`OverflowingMul`]
//!
//! ## Euclidean Operations
//! - [`Euclid`], [`CheckedEuclid`]
//!
//! ## Fused Operations
//! - [`MulAdd`], [`MulAddAssign`]
//!
//! ## Byte Conversion
//! - [`ToBytes`], [`FromBytes`]
//!
//! ## Exponentiation
//! - [`Pow`] with `u8`, `u16`, `u32`, `usize` exponents
//!
//! ## Multiplicative Inverse
//! - [`Inv`] - Returns `Option<UUID>` (`None` for even values)
//!
//! [`Zero`]: num_traits::Zero
//! [`ConstZero`]: num_traits::ConstZero
//! [`One`]: num_traits::One
//! [`ConstOne`]: num_traits::ConstOne
//! [`Bounded`]: num_traits::Bounded
//! [`Num`]: num_traits::Num
//! [`Unsigned`]: num_traits::Unsigned
//! [`PrimInt`]: num_traits::PrimInt
//! [`ToPrimitive`]: num_traits::ToPrimitive
//! [`FromPrimitive`]: num_traits::FromPrimitive
//! [`NumCast`]: num_traits::NumCast
//! [`AsPrimitive`]: num_traits::AsPrimitive
//! [`CheckedAdd`]: num_traits::CheckedAdd
//! [`CheckedSub`]: num_traits::CheckedSub
//! [`CheckedMul`]: num_traits::CheckedMul
//! [`CheckedDiv`]: num_traits::CheckedDiv
//! [`CheckedRem`]: num_traits::CheckedRem
//! [`CheckedNeg`]: num_traits::CheckedNeg
//! [`CheckedShl`]: num_traits::CheckedShl
//! [`CheckedShr`]: num_traits::CheckedShr
//! [`Saturating`]: num_traits::Saturating
//! [`SaturatingAdd`]: num_traits::SaturatingAdd
//! [`SaturatingSub`]: num_traits::SaturatingSub
//! [`SaturatingMul`]: num_traits::SaturatingMul
//! [`WrappingAdd`]: num_traits::WrappingAdd
//! [`WrappingSub`]: num_traits::WrappingSub
//! [`WrappingMul`]: num_traits::WrappingMul
//! [`WrappingNeg`]: num_traits::WrappingNeg
//! [`WrappingShl`]: num_traits::WrappingShl
//! [`WrappingShr`]: num_traits::WrappingShr
//! [`OverflowingAdd`]: num_traits::ops::overflowing::OverflowingAdd
//! [`OverflowingSub`]: num_traits::ops::overflowing::OverflowingSub
//! [`OverflowingMul`]: num_traits::ops::overflowing::OverflowingMul
//! [`Euclid`]: num_traits::Euclid
//! [`CheckedEuclid`]: num_traits::ops::euclid::CheckedEuclid
//! [`MulAdd`]: num_traits::MulAdd
//! [`MulAddAssign`]: num_traits::MulAddAssign
//! [`ToBytes`]: num_traits::ToBytes
//! [`FromBytes`]: num_traits::FromBytes
//! [`Pow`]: num_traits::Pow
//! [`Inv`]: num_traits::Inv

mod bounds;
mod bytes;
mod cast;
mod checked;
mod euclid;
mod identity;
mod inv;
mod mul_add;
mod num;
mod overflowing;
mod pow;
mod prim_int;
mod saturating;
mod wrapping;

#[cfg(test)]
mod tests;
