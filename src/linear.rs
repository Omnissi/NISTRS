extern crate rayon;

use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use statrs::function::gamma::gamma_ur;

use super::*;

/// Linear Complexity Test.
/// The focus of this test is the length of a linear feedback shift register (LFSR). The purpose of this test is to
/// determine whether or not the sequence is complex enough to be considered random. Random sequences
/// are characterized by longer LFSRs. An LFSR that is too short implies non-randomness.
/// `m` The length in bits of a block.
pub fn linear_complexity_test(data: &BitsData, m: usize) -> TestResultT {
    const K: usize = 6;
    const PI: [f64; 7] = [
        0.01047, 0.03125, 0.12500, 0.50000, 0.25000, 0.06250, 0.020833,
    ];

    let n = data.len() / m;
    let sign = match (m + 1) % 2 {
        0 => -1_i8,
        _ => 1_i8,
    };
    let mean = (m as f64) / 2_f64 + (9_f64 + (sign as f64)) / 36_f64
        - 1_f64 / 2_f64.powi(m as i32) * ((m as f64) / 3_f64 + 2_f64 / 9_f64);

    let nu: [AtomicUsize; 7] = Default::default();

    (0..n).into_par_iter().for_each(|i| {
        let mut t = vec![false; m];
        let mut c = vec![false; m];
        let mut b = vec![false; m];

        c[0] = true;
        b[0] = true;

        let mut l = isize::default();
        let mut tmp_m = -1_isize;

        let mut n_ = usize::default();

        while n_ < m {
            let mut const_ind = i * m + n_;
            let mut d = data[const_ind] as usize;

            for j in 1..=(l as usize) {
                if data[const_ind - j] && c[j] {
                    d += 1;
                }
            }

            d %= 2;
            if d == 1 {
                t.copy_from_slice(&c);

                const_ind = match tmp_m {
                    0.. => n_ - (tmp_m as usize),
                    _ => n_ + (-tmp_m as usize),
                };

                for j in 0..(m - const_ind) {
                    c[j + const_ind] ^= b[j];
                }

                if l <= (n_ / 2) as isize {
                    l = (n_ as isize) + 1 - l;
                    tmp_m = n_ as isize;
                    b.copy_from_slice(&t);
                }
            }

            n_ += 1;
        }

        let t = (sign as f64) * ((l as f64) - mean) as f64 + 2_f64 / 9_f64;
        if t <= -2.5 {
            nu[0].fetch_add(1, Ordering::SeqCst);
        } else if t <= -1.5 {
            nu[1].fetch_add(1, Ordering::SeqCst);
        } else if t <= -0.5 {
            nu[2].fetch_add(1, Ordering::SeqCst);
        } else if t <= 0.5 {
            nu[3].fetch_add(1, Ordering::SeqCst);
        } else if t <= 1.5 {
            nu[4].fetch_add(1, Ordering::SeqCst);
        } else if t <= 2.5 {
            nu[5].fetch_add(1, Ordering::SeqCst);
        } else {
            nu[6].fetch_add(1, Ordering::SeqCst);
        }
    });

    let mut chi2 = f64::default();
    for i in 0..(K + 1) {
        chi2 += (nu[i].load(Ordering::SeqCst) as f64 - (n as f64) * PI[i]).powi(2)
            / ((n as f64) * PI[i]);
    }

    let p = gamma_ur((K as f64) / 2_f64, chi2 / 2_f64);

    (p > TEST_THRESHOLD, p)
}
