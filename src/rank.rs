extern crate rayon;

use rayon::prelude::*;

use super::*;
use std::ops::IndexMut;
use std::sync::atomic::{AtomicUsize, Ordering};

const MATRIX_SIZE: usize = 32;

const MINIMAL_BITS: usize = MATRIX_SIZE * MATRIX_SIZE * 38;

type RowsT = [bool; MATRIX_SIZE];
type MatrixT = [RowsT; MATRIX_SIZE];

struct BitsMatrix {
    matrix: MatrixT,
}

/// Binary Matrix Rank Test.
/// The focus of the test is the rank of disjoint sub-matrices of the entire sequence. The purpose of this test is
/// to check for linear dependence among fixed length substrings of the original sequence. Note that this test
/// also appears in the DIEHARD battery of tests.
/// # Number of bits must be greater than 38912!
pub fn rank_test(data: &BitsData) -> Result<TestResultT, String> {
    static P_32: f64 = p_number(32);
    static P_31: f64 = p_number(31);
    static P_30: f64 = 1_f64 - (P_32 + P_31);

    let n_bits = data.len();

    if n_bits < MINIMAL_BITS {
        return Err(format!(
            "{} bit required! In storage: {}",
            MINIMAL_BITS, n_bits
        ));
    }

    let n = n_bits / (MATRIX_SIZE * MATRIX_SIZE);

    let f_31 = AtomicUsize::default();
    let f_32 = AtomicUsize::default();

    (0..n).into_par_iter().for_each(|k| {
        let mut matrix = BitsMatrix::new(data, k);
        match comput_rank(&mut matrix) {
            31 => {
                f_31.fetch_add(1, Ordering::SeqCst);
            }
            32 => {
                f_32.fetch_add(1, Ordering::SeqCst);
            }
            _ => {}
        }
    });

    let f_30 = n - (f_31.load(Ordering::SeqCst) + f_32.load(Ordering::SeqCst));

    let chi_squared = (f_32.load(Ordering::SeqCst) as f64 - (n as f64) * P_32).powi(2)
        / (n as f64 * P_32)
        + (f_31.load(Ordering::SeqCst) as f64 - (n as f64) * P_31).powi(2) / (n as f64 * P_31)
        + (f_30 as f64 - (n as f64) * P_30).powi(2) / (n as f64 * P_30);

    let p = (-chi_squared / 2_f64).exp();

    Ok((p >= TEST_THRESHOLD, p))
}

#[cfg(feature = "const_fn_floating_point_arithmetic")]
#[cfg(feature = "const_for")]
const fn calc_product(r: isize) -> f64 {
    let mut res = 1_f64;

    for i in 0..r - 1 {
        res *= (1_f64 - 2_f64.powf(i as f64 - MATRIX_SIZE as f64)).powf(2_f64)
            / (1_f64 - 2_f64.powf(i as f64 - r as f64));
    }

    res
}

cfg_if::cfg_if! {
    if #[cfg(feature = "const_fn_floating_point_arithmetic")] {
        const fn p_number(r: isize) -> f64 {
            2_f64.powf((r as f64)  * ((MATRIX_SIZE as f64) * (MATRIX_SIZE as f64) - (r as f64))) * calc_product(r)
        }
    } else {
        const fn p_number(r: isize) -> f64 {
            match r {
                31 => 0.577_576_190_173_204_6,
                32 => 0.288_788_095_153_841_1,
                _  => 0_f64
            }
        }
    }
}

impl BitsMatrix {
    fn new(data: &BitsData, k: usize) -> Self {
        let mut res = BitsMatrix {
            matrix: MatrixT::default(),
        };

        let ind_k = k * MATRIX_SIZE * MATRIX_SIZE;
        for i in 0..MATRIX_SIZE {
            let ind_i = i * MATRIX_SIZE;
            for j in 0..MATRIX_SIZE {
                res.matrix[i][j] = data[ind_k + j + ind_i];
            }
        }

        res
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.matrix.swap(i, j)
    }
}

impl Index<usize> for BitsMatrix {
    type Output = RowsT;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        self.matrix.index(i)
    }
}

impl IndexMut<usize> for BitsMatrix {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.matrix.index_mut(i)
    }
}

fn perform_elementary_row_operations_forward(i: usize, matrix: &mut BitsMatrix) {
    for j in (i + 1)..MATRIX_SIZE {
        if matrix[j][i] {
            for k in i..MATRIX_SIZE {
                matrix[j][k] ^= matrix[i][k];
            }
        }
    }
}

fn perform_elementary_row_operations_backward(i: usize, matrix: &mut BitsMatrix) {
    for j in (0..=((i as isize) - 1)).rev() {
        if matrix[j as usize][i] {
            for k in 0..MATRIX_SIZE {
                matrix[j as usize][k] ^= matrix[i][k];
            }
        }
    }
}

fn find_unit_element_and_swap_forward(i: usize, matrix: &mut BitsMatrix) -> bool {
    let mut index = i + 1;
    while index < MATRIX_SIZE && !matrix[index][i] {
        index += 1;
    }

    if index < MATRIX_SIZE {
        matrix.swap(i, index);
        return true;
    }

    false
}

fn find_unit_element_and_swap_backward(i: usize, matrix: &mut BitsMatrix) -> bool {
    let mut index = (i as isize) - 1;
    while index >= 0 && !matrix[index as usize][i] {
        index -= 1;
    }

    if index >= 0 {
        matrix.swap(i, index as usize);
        return true;
    }

    false
}

fn determine_rank(matrix: &BitsMatrix) -> usize {
    let mut rank = MATRIX_SIZE;
    for el in &matrix.matrix {
        if el.iter().all(|x| !(*x)) {
            rank -= 1;
        }
    }

    rank
}

fn comput_rank(matrix: &mut BitsMatrix) -> usize {
    let m = MATRIX_SIZE;
    let tmp = m - 1;

    for i in 0..tmp {
        if matrix[i][i] || find_unit_element_and_swap_forward(i, matrix) {
            perform_elementary_row_operations_forward(i, matrix);
        }
    }

    for i in (0..=tmp).rev() {
        if matrix[i][i] || find_unit_element_and_swap_backward(i, matrix) {
            perform_elementary_row_operations_backward(i, matrix);
        }
    }

    determine_rank(matrix)
}
