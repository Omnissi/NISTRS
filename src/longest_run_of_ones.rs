use statrs::function::gamma::gamma_ur;

use super::*;

const MINIMAL_BITS: usize = 128;

/// Test for the Longest Run of Ones in a Block.
/// The focus of the test is the longest run of ones within M-bit blocks. The purpose of this test is to
/// determine whether the length of the longest run of ones within the tested sequence is consistent with the
/// length of the longest run of ones that would be expected in a random sequence. Note that an irregularity in
/// the expected length of the longest run of ones implies that there is also an irregularity in the expected
/// length of the longest run of zeroes. Therefore, only a test for ones is necessary.
/// # Number of bits must be greater than 128!
/// # Example
/// ```
/// use nistrs::longest_run_of_ones::longest_run_of_ones_test;
/// use nistrs::BitsData;

/// let data = BitsData::from_text("110011000001010101101100010011001110000000000010010011010
///                                 1010001000100111101011010000000110101111100110011100110110110001
///                                 0110010".to_string());
/// assert_eq!(
///     longest_run_of_ones_test(&data).unwrap().1,
///     0.18060931823971144
/// );
pub fn longest_run_of_ones_test(data: &BitsData) -> Result<TestResultT, String> {
    let n_bits = data.len();
    if n_bits < MINIMAL_BITS {
        return Err(format!(
            "{} bit required! In storage: {}",
            MINIMAL_BITS, n_bits
        ));
    }

    let k: usize;
    let m: usize;
    let mut v: [usize; 7] = Default::default();
    let mut nu: [usize; 7] = Default::default();
    let mut pi: [f64; 7] = Default::default();

    if n_bits < 6272 {
        k = 3;
        m = 8;
        v[0] = 1;
        v[1] = 2;
        v[2] = 3;
        v[3] = 4;
        pi[0] = 0.21484375;
        pi[1] = 0.3671875;
        pi[2] = 0.23046875;
        pi[3] = 0.1875;
    } else if n_bits < 750000 {
        k = 5;
        m = 128;
        v[0] = 4;
        v[1] = 5;
        v[2] = 6;
        v[3] = 7;
        v[4] = 8;
        v[5] = 9;
        pi[0] = 0.1174035788;
        pi[1] = 0.242955959;
        pi[2] = 0.249363483;
        pi[3] = 0.17517706;
        pi[4] = 0.102701071;
        pi[5] = 0.112398847;
    } else {
        k = 6;
        m = 10000;
        v[0] = 10;
        v[1] = 11;
        v[2] = 12;
        v[3] = 13;
        v[4] = 14;
        v[5] = 15;
        v[6] = 16;
        pi[0] = 0.0882;
        pi[1] = 0.2092;
        pi[2] = 0.2483;
        pi[3] = 0.1933;
        pi[4] = 0.1208;
        pi[5] = 0.0675;
        pi[6] = 0.0727;
    }

    let n_blocks = n_bits / m;
    for i in 0..n_blocks {
        let mut max_runs = usize::default();
        let mut cur_runs = usize::default();

        for j in 0..m {
            if data[i * m + j] {
                cur_runs += 1;
                max_runs = max_runs.max(cur_runs);
            } else {
                cur_runs = 0;
            }
        }

        if max_runs < v[0] {
            nu[0] += 1;
        }

        for j in 0..=k {
            if max_runs == v[j] {
                nu[j] += 1;
            }
        }

        if max_runs > v[k] {
            nu[k] += 1;
        }
    }

    let mut chi2 = f64::default();
    for i in 0..=k {
        chi2 += ((nu[i] as f64) - (n_blocks as f64) * pi[i]).powi(2) / ((n_blocks as f64) * pi[i]);
    }

    let p = gamma_ur((k as f64) / 2_f64, chi2 / 2_f64);

    Ok((p >= TEST_THRESHOLD, p))
}
