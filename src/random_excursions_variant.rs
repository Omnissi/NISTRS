use libm::erfc;

use super::*;

/// Random Excursions Variant Test.
///  The focus of this test is the total number of times that a particular state is visited (i.e., occurs) in a
/// cumulative sum random walk. The purpose of this test is to detect deviations from the expected number
/// of visits to various states in the random walk. This test is actually a series of eighteen tests (and
/// conclusions), one test and conclusion for each of the states: -9, -8, …, -1 and +1, +2, …, +9.
///
/// Return `P-value` for 18 state: [-9, -8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9].
pub fn random_excursions_variant_test(data: &BitsData) -> Result<[TestResultT; 18], String> {
    const STATE_X: [isize; 18] = [
        -9, -8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    ];

    let n = data.len();

    let mut s_k = vec![isize::default(); n];
    s_k[0] = 2 * (data[0] as isize) - 1;

    let mut j = usize::default();
    for i in 1..n {
        s_k[i] = s_k[i - 1] + 2 * (data[i] as isize) - 1;
        if s_k[i] == 0 {
            j += 1;
        }
    }

    if s_k[n - 1] != 0 {
        j += 1;
    }

    if (j as f64) < (0.005 * (n as f64).sqrt()).max(500_f64) {
        return Err(
            "WARNING:  TEST NOT APPLICABLE.  THERE ARE AN INSUFFICIENT NUMBER OF CYCLES."
                .to_string(),
        );
    }

    let mut res: [TestResultT; 18] = Default::default();
    for i in 0..STATE_X.len() {
        let x = STATE_X[i];
        let count = s_k.iter().filter(|v| **v == x).count();

        let arg = ((count as f64) - (j as f64)).abs()
            / (2_f64 * (j as f64) * (4_f64 * (x.abs() as f64) - 2_f64)).sqrt();
        let p = erfc(arg);
        res[i] = (p >= TEST_THRESHOLD, p);
    }

    Ok(res)
}
