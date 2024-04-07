extern crate num;
use num::complex::Complex;
use fft::{fft_simd_f64x2, FastFourierTransform};

/*
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
Installing Rust nightly to use those features, commands below:
   rustup default nightly
*/

fn main() {
    let output = fft(vec![5.0, 3.0, 2.0, 1.0]);
    dbg!(output);

    let output_simd = fft_simd_f64x2(vec![5.0, 3.0, 2.0, 1.0]);
    dbg!(output_simd);
}

/// Performs Fast Fourier Transform (FFT) on the input vector.
/// 
/// # Arguments
/// 
/// * `input` - A vector of real numbers representing the input signal.
/// 
/// # Returns
/// 
/// A vector of complex numbers representing the FFT output.
/// 
/// # Example
/// 
/// ```
/// use num::complex::Complex;
/// let output = fft(vec![5.0, 3.0, 2.0, 1.0]);
/// assert_eq!(output[0], Complex::new(11.0, 0.0));
/// assert_eq!(output[1], Complex::new(3.0, -2.0));
/// assert_eq!(output[2], Complex::new(3.0, 0.0));
/// assert_eq!(output[3], Complex::new(3.0, 2.0));
/// ```
pub fn fft(input: Vec<f64>) -> Vec<Complex<f64>> {
    let transform = FastFourierTransform::new(input.clone());
    let mut vec: Vec<Complex<f64>> = input
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    transform.fft_rec(&mut vec);
    vec
}

