use std::sync::Arc;

use super::*;

use libm::erfc;
use rustfft::{num_complex::Complex, Fft, FftPlanner, FftPlannerAvx, FftPlannerSse};

/// Discrete Fourier Transform (Spectral) Test.
/// The focus of this test is the peak heights in the Discrete Fourier Transform of the sequence. The purpose
/// of this test is to detect periodic features (i.e., repetitive patterns that are near each other) in the tested
/// sequence that would indicate a deviation from the assumption of randomness. The intention is to detect
/// whether the number of peaks exceeding the 95 % threshold is significantly different than 5 %.
pub fn fft_test(data: &BitsData) -> TestResultT {
    let n = data.len();

    type FftType = f64;

    let fft: Arc<dyn Fft<FftType>>;

    if let Ok(mut plan) = FftPlannerAvx::<FftType>::new() {
        fft = plan.plan_fft_forward(n);
    } else if let Ok(mut plan) = FftPlannerSse::<FftType>::new() {
        fft = plan.plan_fft_forward(n);
    } else {
        let mut planner = FftPlanner::<FftType>::new();
        fft = planner.plan_fft_forward(n);
    }

    let mut buf = Vec::<Complex<FftType>>::with_capacity(n);

    data.iter().for_each(|x| {
        if *x {
            buf.push(Complex {
                re: 1_f64,
                im: 0_f64,
            });
        } else {
            buf.push(Complex {
                re: -1_f64,
                im: 0_f64,
            });
        }
    });

    fft.process(&mut buf);

    let mut count = 0_usize;
    let upper_bound = (2.995732274 * (n as f64)).sqrt();

    for i in buf.iter().take(n / 2) {
        if i.norm() < upper_bound {
            count += 1;
        }
    }

    let d = (count as f64 - 0.95 * n as f64 / 2_f64) / (n as f64 / 4.0 * 0.95 * 0.05).sqrt();
    let p = erfc(d.abs() / 2_f64.sqrt());

    (p > TEST_THRESHOLD, p)
}
