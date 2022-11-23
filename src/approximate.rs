use statrs::function::gamma::gamma_ur;

use super::*;

/// Approximate Entropy Test.
/// As with the Serial test, the focus of this test is the frequency of all possible overlapping
/// m-bit patterns across the entire sequence. The purpose of the test is to compare the frequency of
/// overlapping blocks of two consecutive/adjacent lengths (m and m+1) against the expected result for a
/// random sequence.
/// `m` the length of each block – in this case, the first block length used in the test. m+1 is the second block length used.
pub fn approximate_entropy_test(data: &BitsData, m: usize) -> TestResultT {
    let n = data.len();

    let mut ap_en = [f64::default(); 2];
    let mut r = usize::default();

    for block_size in m..=(m + 1) {
        if block_size == 0 {
            ap_en[0] = 0_f64;
            r += 1;
            continue;
        }

        let pow_len = 2_usize.pow(block_size as u32 + 1) - 1;
        let mut p = vec![usize::default(); pow_len];

        for i in 0..n {
            let mut k = 1_usize;
            for j in 0..block_size {
                k <<= 1;
                if data[(i + j) % n] {
                    k += 1;
                }
            }

            p[k - 1] += 1;
        }

        let pow_const = 2_usize.pow(block_size as u32);
        let mut index = pow_const - 1;
        let mut sum = f64::default();
        for _ in 0..pow_const {
            if p[index] > 0 {
                sum += (p[index] as f64) * ((p[index] as f64) / (n as f64)).ln();
            }

            index += 1;
        }

        sum /= n as f64;
        ap_en[r] = sum;
        r += 1;
    }

    let apen = ap_en[0] - ap_en[1];
    let chi2 = 2_f64 * (n as f64) * (2_f64.ln() - apen);
    let p = gamma_ur(2_f64.powi(m as i32 - 1), chi2 / 2_f64);

    (p >= TEST_THRESHOLD, p)
}
