use std::cmp::max;

use libm::erf;

use super::*;

/// Cumulative Sums (Cusum) Test.
/// The focus of this test is the maximal excursion (from zero) of the random walk defined by the cumulative
/// sum of adjusted (-1, +1) digits in the sequence. The purpose of the test is to determine whether the
/// cumulative sum of the partial sequences occurring in the tested sequence is too large or too small relative
/// to the expected behavior of that cumulative sum for random sequences. This cumulative sum may be
/// considered as a random walk. For a random sequence, the excursions of the random walk should be near
/// zero. For certain types of non-random sequences, the excursions of this random walk from zero will be
/// large.
/// Return `P-values` for cusum-forward and cusum-reverse.
pub fn cumulative_sums_test(data: &BitsData) -> [TestResultT; 2] {
    let mut s = isize::default();
    let mut sup = isize::default();
    let mut inf = isize::default();
    let mut z = isize::default();
    let mut zrev = isize::default();

    for el in data.iter() {
        if *el {
            s += 1;
        } else {
            s -= 1;
        }

        if s > sup {
            sup += 1;
        }

        if s < inf {
            inf -= 1;
        }

        z = max(sup, -inf);
        zrev = max(sup - s, s - inf);
    }

    let n = data.len();
    let sqrtn = (n as f64).sqrt();

    let mut begin = (-(n as isize) / z + 1) / 4;
    let mut end = (n as isize / z - 1) / 4;

    let mut sum1 = f64::default();
    for k in begin..=end {
        sum1 += normal(((4 * k + 1) * z) as f64 / sqrtn);
        sum1 -= normal(((4 * k - 1) * z) as f64 / sqrtn);
    }

    let mut sum2 = f64::default();
    for k in ((-(n as isize) / z - 3) / 4)..=end {
        sum2 += normal(((4 * k + 3) * z) as f64 / sqrtn);
        sum2 -= normal(((4 * k + 1) * z) as f64 / sqrtn);
    }

    let p0 = 1_f64 - sum1 + sum2;

    begin = (-(n as isize) / zrev + 1) / 4;
    end = (n as isize / zrev - 1) / 4;

    sum1 = 0_f64;
    for k in begin..=end {
        sum1 += normal(((4 * k + 1) * zrev) as f64 / sqrtn);
        sum1 -= normal(((4 * k - 1) * zrev) as f64 / sqrtn);
    }

    sum2 = f64::default();
    for k in ((-(n as isize) / zrev - 3) / 4)..=end {
        sum2 += normal(((4 * k + 3) * zrev) as f64 / sqrtn);
        sum2 -= normal(((4 * k + 1) * zrev) as f64 / sqrtn);
    }

    let p1 = 1_f64 - sum1 + sum2;

    [(p0 >= TEST_THRESHOLD, p0), (p1 >= TEST_THRESHOLD, p1)]
}

fn normal(x: f64) -> f64 {
    let sqrt2: f64 = 2_f64.sqrt();

    (1_f64 + erf(x / sqrt2)) / 2_f64
}
