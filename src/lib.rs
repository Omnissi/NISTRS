//! This crate implements statistical tests according to the [NIST standard](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-22r1a.pdf).
//!
//! # Example usage:
//!
//! ```
//! use nistrs::prelude::*;
//!
//! let data = BitsData::from_binary(vec!(0x23, 0x44));
//! let result = frequency_test(&data);
//! print!("Test passed: {}; P-value: {}", result.0, result.1);
//! ```
use core::slice::Iter;
use std::{ops::Index, slice::SliceIndex};

pub mod approximate;
pub mod block_freq;
pub mod cusum;
pub mod fft;
pub mod freq;
pub mod linear;
pub mod longest_run_of_ones;
pub mod non_overlapping_template;
pub mod overlapping_template;
pub mod random_excursions;
pub mod random_excursions_variant;
pub mod rank;
pub mod runs;
pub mod serial;
pub mod universal;

pub const TEST_THRESHOLD: f64 = 0.01;

/// Return type for most tests.
/// 0 - test passed, 1 - P-value.
pub type TestResultT = (bool, f64);

type BitsT = Vec<bool>;

/// Structure contained sequence of bit.
pub struct BitsData {
    ones: usize,
    data: BitsT,
}

impl BitsData {
    /// Transform byte-vector into a `BitsData`.
    /// # Example
    ///
    /// ```
    /// use nistrs::BitsData;
    ///
    /// let result = BitsData::from_binary(vec![0x12, 0x21]);
    /// ```
    pub fn from_binary(data: Vec<u8>) -> Self {
        let mut res = BitsData {
            ones: 0,
            data: BitsT::with_capacity(data.len() * (u8::BITS as usize)),
        };

        for i in data {
            res.ones += i.count_ones() as usize;
            for n in (0..(u8::BITS as isize)).rev() {
                res.data.push((i >> n & 1) == 1);
            }
        }

        res
    }

    /// Transform a string consisting of `0` and `1` into a `BitsData`.
    /// # Panic
    /// Symbol not `0` or `1`.
    /// # Example
    ///
    /// ```
    /// use nistrs::BitsData;
    ///
    /// let result = BitsData::from_text("0010010001010001".to_string());
    /// ```
    pub fn from_text(data: String) -> Self {
        let mut res = BitsData {
            ones: 0,
            data: BitsT::with_capacity(data.len()),
        };

        for i in data.lines() {
            i.trim().chars().for_each(|x| match x.to_digit(2) {
                Some(v) => {
                    if v == 1 {
                        res.ones += 1;
                    }
                    res.data.push(v == 1);
                }
                None => {
                    panic!("incorrect text data!")
                }
            })
        }

        res
    }

    /// Return number of bits in storage.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check that storage of bits is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the iterator for bit sequence.
    #[inline]
    pub fn iter(&self) -> Iter<bool> {
        self.data.iter()
    }

    /// Return number of `1` in sequence.
    #[inline]
    pub fn ones(&self) -> usize {
        self.ones
    }
}

impl<I> Index<I> for BitsData
where
    I: SliceIndex<[bool]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

/// The module imports all NIST tests in library.
pub mod prelude {
    pub use crate::{
        approximate::approximate_entropy_test, block_freq::block_frequency_test,
        cusum::cumulative_sums_test, fft::fft_test, freq::frequency_test,
        linear::linear_complexity_test, longest_run_of_ones::longest_run_of_ones_test,
        non_overlapping_template::non_overlapping_template_test,
        overlapping_template::overlapping_template_test, random_excursions::random_excursions_test,
        random_excursions_variant::random_excursions_variant_test, rank::rank_test,
        runs::runs_test, serial::serial_test, universal::universal_test, *,
    };
}
#[cfg(test)]
mod tests {
    use crate::BitsData;

    #[test]
    #[rustfmt::skip]
    fn from_binary() {
        let result = BitsData::from_binary(vec![0x12, 0x21]);
        assert!(result.data.eq(&[
            false, false, false, true, 
            false, false, true,  false, 
            false, false, true,  false,
            false, false, false, true
        ]));
    }

    #[test]
    #[rustfmt::skip]
    fn from_text() {
        let result = BitsData::from_text("0010010001010001".to_string());
        assert!(result.data.eq(&[
            false, false, true,  false, 
            false, true,  false, false, 
            false, true,  false, true,
            false, false, false, true
        ]))
    }

    #[test]
    #[should_panic]
    #[rustfmt::skip]
    fn from_error_text() {
        BitsData::from_text("0010020001010001".to_string());
    }
}
