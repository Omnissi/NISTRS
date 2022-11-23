extern crate rayon;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use statrs::function::gamma::gamma_ur;

use super::*;

/// Serial Test.
/// The focus of this test is the frequency of all possible overlapping m-bit patterns across the entire
/// sequence. The purpose of this test is to determine whether the number of occurrences of the 2m m-bit
/// overlapping patterns is approximately the same as would be expected for a random sequence. Random
/// sequences have uniformity; that is, every m-bit pattern has the same chance of appearing as every other
/// m-bit pattern. Note that for m = 1, the Serial test is equivalent to the Frequency test.
/// `m` the length in bits of a block.
pub fn serial_test(data: &BitsData, m: usize) -> [TestResultT; 2] {
    let psi: Vec<_> = (0..3_usize)
        .into_par_iter()
        .map(|i| psi2(data, m - i))
        .collect();

    let del1 = psi[0] - psi[1];
    let del2 = psi[0] - 2_f64 * psi[1] + psi[2];

    let p = [
        gamma_ur(2_f64.powi(m as i32 - 1) / 2_f64, del1 / 2_f64),
        gamma_ur(2_f64.powi(m as i32 - 2) / 2_f64, del2 / 2_f64),
    ];

    [
        (p[0] >= TEST_THRESHOLD, p[0]),
        (p[1] >= TEST_THRESHOLD, p[1]),
    ]
}

#[inline]
fn psi2(data: &BitsData, m: usize) -> f64 {
    if m == 1 {
        return 0_f64;
    }

    let pow_len = 2_usize.pow(m as u32 + 1) - 1;
    let mut p = vec![usize::default(); pow_len];

    let n = data.len();
    for i in 0..n {
        let mut k = 1_usize;
        for j in 0..m {
            if data[(i + j) % n] {
                k = k * 2 + 1;
            } else {
                k *= 2;
            }
        }

        p[k - 1] += 1;
    }

    let mut sum = f64::default();
    for i in p.iter().take(pow_len).skip(2_usize.pow(m as u32) - 1) {
        sum += i.pow(2) as f64;
    }

    (sum * (2_usize.pow(m as u32) as f64) / (n as f64)) - (n as f64)
}
