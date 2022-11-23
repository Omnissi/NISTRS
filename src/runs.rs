use libm::erfc;

use super::*;

/// Runs Test.
/// The focus of this test is the total number of runs in the sequence, where a run is an uninterrupted sequence
/// of identical bits. A run of length k consists of exactly k identical bits and is bounded before and after with
/// a bit of the opposite value. The purpose of the runs test is to determine whether the number of runs of
/// ones and zeros of various lengths is as expected for a random sequence. In particular, this test determines
/// whether the oscillation between such zeros and ones is too fast or too slow.
/// # Example
/// ```
/// use nistrs::runs::runs_test;
/// use nistrs::BitsData;
///
/// let data = BitsData::from_text("11001001000011111101101010100010001000010110100011000010001101001100010011000110011000101000101110000000".to_string());
/// assert_eq!(runs_test(&data).1, 0.6953317934158357);
/// ```
pub fn runs_test(data: &BitsData) -> TestResultT {
    let n_bits = data.len();
    let n_ones = data.ones();

    let pi = (n_ones as f64) / (n_bits as f64);
    if (pi - 0.5).abs() > (2.0 / (n_bits as f64).sqrt()) {
        return (false, 0.5);
    }

    let mut v = 1_usize;
    for i in 1..n_bits {
        if data[i] != data[i - 1] {
            v += 1;
        }
    }

    let erfc_arg = ((v as f64) - 2_f64 * (n_bits as f64) * pi * (1_f64 - pi)).abs()
        / (2_f64 * pi * (1_f64 - pi) * (2_f64 * (n_bits as f64)).sqrt());
    let p = erfc(erfc_arg);

    (p >= TEST_THRESHOLD, p)
}
