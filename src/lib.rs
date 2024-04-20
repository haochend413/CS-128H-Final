#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]
// #[macro_use]

/*
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
Installing Rust nightly to use those features, commands:
   rustup default nightly
*/

extern crate num;
use std::f64::consts::PI; 
// use std::{f64::consts::PI,simd::Simd};
use num::complex::{Complex, ComplexFloat};
// use std::simd::f64x2;
// use itertools::{Itertools, Either};

//only implemented for base case where data.len() == 4 (for fully simd operation)
//see: https://doc.rust-lang.org/std/simd/type.f64x2.html
// pub fn fft_simd_f64x2(data: Vec<f64>) -> Vec<(Simd<f64, 2>, Simd<f64, 2>)>{
//     let size = data.len();
//     let half = size / 2;

//     let complex_vector: Vec<_> = (0..half)
//         .map(|x| Complex::new(0.0, -2.0 * PI * x as f64 / size as f64).exp())
//         .collect();

//     // when size == 4
//     // d0 + d1, d0 - d1
//     let (left, right) = data.split_at(half);
//     let base_case:Vec<_>= left.array_chunks::<2>()
//         .map(|&left| f64x2::from_array(left))
//         .zip(right.array_chunks::<2>().map(|&right| f64x2::from_array(right)))
//         .flat_map(|(left, right)| vec![((left+right), f64x2::splat(0.0)), ((left-right), f64x2::splat(0.0))])
//         .collect();

//     // dbg!(base_case.clone());

//     // complex<f64x2>
//     let real= [complex_vector[0].re(), complex_vector[1].re()];
//     let imagine= [complex_vector[0].im(), complex_vector[1].im()];
//     let complex = (f64x2::from_array(real), f64x2::from_array(imagine));

//     // odd<f64x2> & even<f64x2>
//     // PS: odd/even index, start at 0 not 1!
//     let (even, odd) = 
//         (((base_case[0].0.interleave(base_case[1].0).0), (base_case[0].1.interleave(base_case[1].1).0)),
//         ((base_case[0].0.interleave(base_case[1].0).1), (base_case[0].1.interleave(base_case[1].1).1)));

//     // dbg!(even, odd);
        
//     // odd<f64x2> * complex<f64x2> = tmp<f64x2>
//     // did not found complex * complex in portable_simd, implemented manually
//     let temp = 
//         ((odd.0 * complex.0 - odd.1 * complex.1), (odd.0 * complex.1 + odd.1 * complex.0));

//     // result[0 & 1] = even<f64x2> + tmp<f64x2>
//     let a = (even.0 + temp.0).interleave(even.1 + temp.1);
//     // result[2 & 3] = even<f64x2> - tmp<f64x2>
//     let b = (even.0 - temp.0).interleave(even.1 - temp.1);

//     vec![a, b]

// }


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
