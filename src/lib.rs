extern crate num;
use std::f64::consts::PI;
use num::complex::Complex;
#[derive(Debug)]
pub struct FastFourierTransform {
    pub complex_vector: Vec<Complex<f64>>,
    pub size: usize,
}

impl FastFourierTransform {
    /// Creates a new instance of FastFourierTransform.
    /// 
    /// # Arguments
    /// 
    /// * `data` - A vector of real numbers representing the input signal.
    pub fn new(data: Vec<f64>) -> FastFourierTransform {
        let size = data.len();
        let mut complex_vector = Vec::with_capacity(size);
        for x in 0..size {
            let w = Complex::new(0.0, -2.0 * PI * x as f64 / size as f64).exp();
            complex_vector.push(w);
        }

        FastFourierTransform {
            complex_vector,
            size,
        }
    }

    /// Recursive function to perform FFT.
    /// 
    /// # Arguments
    /// 
    /// * `data` - A mutable reference to a vector of complex numbers.
    pub fn fft_rec(&self, data: &mut Vec<Complex<f64>>) {
        //make up zeros
        let mut p = 1;
        let mut num_to_add = 0;  
        loop {
            let a = usize::pow(2, p);
            if a < data.len() {
                p += 1;
            } else {
                num_to_add = usize::pow(2, p) - data.len();
                break;
            }
        }
        
        for i in 0..num_to_add{
            data.push(Complex::new(0.0, 0.0)); 
        }

        let n = data.len();
        
        if n == 2 {
            let d0 = data[0];
            let d1 = data[1];
            data[0] = d0 + d1;
            data[1] = d0 - d1;
            return;
        }

        let half = n / 2;
        let mut vec_even = vec![Complex::new(0.0, 0.0); half];
        let mut vec_odd: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); half];

        for i in 0..half {
            vec_even[i] = data[2 * i];
            vec_odd[i] = data[2 * i + 1];
        }

        self.fft_rec(&mut vec_even);
        self.fft_rec(&mut vec_odd);

        for x in 0..half {
            let index = self.size / n * x;
            let complex = self.complex_vector[index] * vec_odd[x]; 
            data[x] = vec_even[x] + complex;
            data[x + half] = vec_even[x] - complex;
        }
    }
}
