extern crate rayon;
use statrs::function::gamma::gamma_ur;

mod template10;
mod template11;
mod template12;
mod template13;
mod template14;
mod template15;
mod template16;
mod template2;
mod template3;
mod template4;
mod template5;
mod template6;
mod template7;
mod template8;
mod template9;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use self::{
    template10::TEMPLATE10, template11::TEMPLATE11, template12::TEMPLATE12, template13::TEMPLATE13,
    template14::TEMPLATE14, template15::TEMPLATE15, template16::TEMPLATE16, template2::TEMPLATE2,
    template3::TEMPLATE3, template4::TEMPLATE4, template5::TEMPLATE5, template6::TEMPLATE6,
    template7::TEMPLATE7, template8::TEMPLATE8, template9::TEMPLATE9,
};

use super::*;

const MAX_NUM_OF_TEMPLATES: usize = 148;

/// Non-overlapping Template Matching Test.
/// The focus of this test is the number of occurrences of pre-specified target strings. The purpose of this
/// test is to detect generators that produce too many occurrences of a given non-periodic (aperiodic) pattern.
/// For this test and for the Overlapping Template Matching test, an m-bit window is used to
/// search for a specific m-bit pattern. If the pattern is not found, the window slides one bit position. If the
/// pattern is found, the window is reset to the bit after the found pattern, and the search resumes.
/// `m` The length in bits of each template. (2 <= `m` <= 16).
pub fn non_overlapping_template_test(
    data: &BitsData,
    m: usize,
) -> Result<Vec<TestResultT>, String> {
    if !(2..=16).contains(&m) {
        return Err("In this implementation 2 <= m <= 16 !".to_string());
    }

    const N: usize = 8;
    let n_bits = data.len();
    let m_blocks = n_bits / N;

    let lambda = ((m_blocks - m + 1) as f64) / 2_f64.powi(m as i32);
    let sqr_var_wj = ((m_blocks as f64)
        * (1_f64 / 2_f64.powi(m as i32)
            - (2_f64 * (m as f64) - 1_f64) / 2_f64.powf(2_f64 * (m as f64))))
    .sqrt();

    let num_of_rows = get_template_size(m).min(MAX_NUM_OF_TEMPLATES);

    let par_iter = (0..num_of_rows).into_par_iter().map(|i| {
        let mut wj = Vec::<usize>::new();
        wj.resize(N, usize::default());

        let seq = get_tempalte(i, m);

        for (j, cnt) in wj.iter_mut().enumerate().take(N) {
            let mut begin = j * m_blocks;
            let mut end = begin + m;

            let mut w_obs = usize::default();
            let mut k = 0_usize;
            while k < (m_blocks - m + 1) {
                if data[begin..end].eq(seq) {
                    w_obs += 1;
                    begin += m - 1;
                    end += m - 1;
                    k += m - 1;
                }
                begin += 1;
                end += 1;
                k += 1;
            }
            *cnt = w_obs;
        }

        let mut chi2 = 0_f64;
        for &j in wj.iter().take(N) {
            chi2 += (((j as f64) - lambda) / sqr_var_wj).powi(2);
        }

        let p = gamma_ur(N as f64 / 2_f64, chi2 / 2_f64);
        (p >= TEST_THRESHOLD, p)
    });

    Ok(par_iter.collect())
}

fn get_tempalte(i: usize, m: usize) -> &'static [bool] {
    match m {
        2 => &TEMPLATE2[i],
        3 => &TEMPLATE3[i],
        4 => &TEMPLATE4[i],
        5 => &TEMPLATE5[i],
        6 => &TEMPLATE6[i],
        7 => &TEMPLATE7[i],
        8 => &TEMPLATE8[i],
        9 => &TEMPLATE9[i],
        10 => &TEMPLATE10[i],
        11 => &TEMPLATE11[i],
        12 => &TEMPLATE12[i],
        13 => &TEMPLATE13[i],
        14 => &TEMPLATE14[i],
        15 => &TEMPLATE15[i],
        16 => &TEMPLATE16[i],
        _ => panic!("Unknown size! M = {}", m),
    }
}

fn get_template_size(m: usize) -> usize {
    match m {
        2 => TEMPLATE2.len(),
        3 => TEMPLATE3.len(),
        4 => TEMPLATE4.len(),
        5 => TEMPLATE5.len(),
        6 => TEMPLATE6.len(),
        7 => TEMPLATE7.len(),
        8 => TEMPLATE8.len(),
        9 => TEMPLATE9.len(),
        10 => TEMPLATE10.len(),
        11 => TEMPLATE11.len(),
        12 => TEMPLATE12.len(),
        13 => TEMPLATE13.len(),
        14 => TEMPLATE14.len(),
        15 => TEMPLATE15.len(),
        16 => TEMPLATE16.len(),
        _ => panic!("Unknown size! M = {}", m),
    }
}
