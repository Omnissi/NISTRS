use libm::erfc;

use super::*;

/// Frequency (Monobit) Test.
///
/// The focus of the test is the proportion of zeroes and ones for the entire sequence. The purpose of this test
/// is to determine whether the number of ones and zeros in a sequence are approximately the same as would
/// be expected for a truly random sequence. The test assesses the closeness of the fraction of ones to 1/2, that
/// is, the number of ones and zeroes in a sequence should be about the same. All subsequent tests depend on
/// the passing of this test.
/// # Example
/// ```
/// use nistrs::freq::frequency_test;
/// use nistrs::BitsData;

/// let data = BitsData::from_text("11001001000011111101101010100010001000010110100011000010001101001100010011000110011000101000101110001100".to_string());
/// assert_eq!(frequency_test(&data).1, 0.11666446478102338);
/// ```
pub fn frequency_test(data: &super::BitsData) -> TestResultT {
    let ones = data.ones();
    let nbits = data.len();

    let sn = 2 * ones as isize - nbits as isize;
    let sobs = sn.abs() as f64 / (nbits as f64).sqrt();
    let p = erfc(sobs / 2.0_f64.sqrt());

    (p >= TEST_THRESHOLD, p)
}
