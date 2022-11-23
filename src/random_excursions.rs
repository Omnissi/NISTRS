use statrs::function::gamma::gamma_ur;

use super::*;

/// Random Excursions Test.
///  The focus of this test is the number of cycles having exactly K visits in a cumulative sum random walk.
/// The cumulative sum random walk is derived from partial sums after the (0,1) sequence is transferred to
/// the appropriate (-1, +1) sequence. A cycle of a random walk consists of a sequence of steps of unit length
/// taken at random that begin at and return to the origin. The purpose of this test is to determine if the
/// number of visits to a particular state within a cycle deviates from what one would expect for a random
/// sequence. This test is actually a series of eight tests (and conclusions), one test and conclusion for each of
/// the states: -4, -3, -2, -1 and +1, +2, +3, +4.
///
/// Return `P-value` for 8 state: [-4, -3, -2, -1, 1, 2, 3, 4].
pub fn random_excursions_test(data: &BitsData) -> Result<[TestResultT; 8], String> {
    const STATE_X: [isize; 8] = [-4, -3, -2, -1, 1, 2, 3, 4];
    const PI: [[f64; 6]; 5] = [
        [
            0.0000000000,
            0.00000000000,
            0.00000000000,
            0.00000000000,
            0.00000000000,
            0.0000000000,
        ],
        [
            0.5000000000,
            0.25000000000,
            0.12500000000,
            0.06250000000,
            0.03125000000,
            0.0312500000,
        ],
        [
            0.7500000000,
            0.06250000000,
            0.04687500000,
            0.03515625000,
            0.02636718750,
            0.0791015625,
        ],
        [
            0.8333333333,
            0.02777777778,
            0.02314814815,
            0.01929012346,
            0.01607510288,
            0.0803755143,
        ],
        [
            0.8750000000,
            0.01562500000,
            0.01367187500,
            0.01196289063,
            0.01046752930,
            0.0732727051,
        ],
    ];

    let n = data.len();
    let max_iteration = 1000.max(n);

    let mut s_k = vec![isize::default(); n];
    s_k[0] = 2 * (data[0] as isize) - 1;

    let mut cycle = vec![usize::default(); max_iteration];
    let mut j = usize::default();
    for i in 1..data.len() {
        s_k[i] = s_k[i - 1] + 2 * (data[i] as isize) - 1;
        if s_k[i] == 0 {
            j += 1;
            if j > max_iteration {
                return Err(
                    "In RandomExcursionsTest:  EXCEEDING THE MAX NUMBER OF CYCLES EXPECTED"
                        .to_string(),
                );
            }

            cycle[j] = i;
        }
    }

    if s_k[n - 1] != 0 {
        j += 1;
    }

    cycle[j] = data.len();

    if (j as f64) < (0.005 * (n as f64).sqrt()).max(500_f64) {
        return Err(
            "WARNING:  TEST NOT APPLICABLE.  THERE ARE AN INSUFFICIENT NUMBER OF CYCLES."
                .to_string(),
        );
    }

    let mut nu: [[usize; 8]; 6] = Default::default();

    let mut cycle_start = usize::default();
    let mut cycle_end = cycle[1];

    for i in 1..=j {
        let mut counter: [usize; 8] = Default::default();
        for k in cycle_start..cycle_end {
            if (s_k[k] >= 1 && s_k[k] <= 4) || (s_k[k] >= -4 && s_k[k] <= -1) {
                if s_k[k] < 0 {
                    counter[(s_k[k] + 4) as usize] += 1;
                } else {
                    counter[(s_k[k] + 3) as usize] += 1;
                }
            }
        }

        cycle_start = cycle[i] + 1;
        if i < j {
            cycle_end = cycle[i + 1];
        }

        for (k, it) in counter.iter().enumerate() {
            let index = it.min(&5);
            nu[*index][k] += 1;
        }
    }

    let mut p = [TestResultT::default(); 8];

    for i in 0..STATE_X.len() {
        let pi_x = &PI[STATE_X[i].unsigned_abs()];
        let mut sum = f64::default();

        for k in 0..nu.len() {
            let tmp = (j as f64) * pi_x[k];
            sum += (nu[k][i] as f64 - tmp).powi(2) / tmp;
        }

        let tmp_p = gamma_ur(2.5, sum / 2_f64);
        p[i] = (tmp_p >= TEST_THRESHOLD, tmp_p);
    }

    Ok(p)
}
