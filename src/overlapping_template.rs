use libm::lgamma;
use statrs::function::gamma::gamma_ur;

use super::*;

/// Overlapping Template Matching Test.
/// The focus of the Overlapping Template Matching test is the number of occurrences of pre-specified target
/// strings. Both this test and the Non-overlapping Template Matching test use an m-bit
/// window to search for a specific m-bit pattern. If the pattern is not found,
/// the window slides one bit position. The difference between this test and the test in Non-overlapping Template Matching test is that
/// when the pattern is found, the window slides only one bit before resuming the search.
/// `m` - the length in bits of each template.
pub fn overlapping_template_test(data: &BitsData, m: usize) -> TestResultT {
    const M: usize = 1032;

    let n = data.len() / M;

    let pi = compute_pi(M, m);

    let test_seq = vec![true; m];
    let mut nu: [f64; 6] = [0_f64; 6];
    let mut begin = usize::default();
    let mut end = begin + m;

    for _ in 0..n {
        let mut w_obs = usize::default();
        for _ in 0..(M - m + 1) {
            if data[begin..end].eq(&test_seq) {
                w_obs += 1;
            }
            begin += 1;
            end += 1;
        }

        begin += m - 1;
        end += m - 1;
        w_obs = w_obs.min(nu.len() - 1);
        nu[w_obs] += 1_f64;
    }
    let mut chi2 = 0_f64;
    nu.iter().zip(pi.iter()).for_each(|x| {
        chi2 += (x.0 - x.1 * (n as f64)).powi(2) / (x.1 * (n as f64));
    });

    let p = gamma_ur(2.5_f64, chi2 / 2_f64);

    (p >= TEST_THRESHOLD, p)
}

fn compute_pi(c_m: usize, m: usize) -> [f64; 6] {
    let lambda = (c_m - m + 1) as f64 / 2_f64.powi(m as i32);
    let eta = lambda / 2_f64;

    let mut sum = 0_f64;
    let mut res: [f64; 6] = [0_f64; 6];

    for (i, it) in res.iter_mut().enumerate().take(5) {
        let tmp = single_compute_pi(i, eta);
        sum += tmp;

        *it = tmp;
    }

    res[5] = 1_f64 - sum;

    res
}

fn single_compute_pi(u: usize, eta: f64) -> f64 {
    if u == 0 {
        return (-eta).exp();
    }

    let mut sum = 0_f64;
    for l in 1..=u {
        sum += (-eta - (u as f64) * 2_f64.ln() + (l as f64) * eta.ln() - lgamma(l as f64 + 1_f64)
            + lgamma(u as f64)
            - lgamma(l as f64)
            - lgamma((u as f64) - (l as f64) + 1_f64))
        .exp();
    }

    sum
}
