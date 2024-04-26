#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]
// #[macro_use]

extern crate num;

use std::{f64::consts::PI, simd::Simd, vec};
use num::complex::{Complex, ComplexFloat};
use std::simd::f64x2;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};


#[derive(Copy, Debug, Clone)]
pub struct SimdComplex {
    pub re: Simd<f64, 2>,
    pub im: Simd<f64, 2>,
}

// impl SimdComplex {
//     pub fn new(re: Simd<f64, 2>, im: Simd<f64, 2>) -> SimdComplex{
//         SimdComplex{re, im}
//     }
// }

fn mul(a: SimdComplex, b: SimdComplex)-> SimdComplex{
    SimdComplex {
        re: a.re*b.re - a.im*b.im,
        im: a.re*b.im + a.im*b.re,
    }
}

fn sub(a: SimdComplex, b: SimdComplex)-> SimdComplex {
    SimdComplex {
        re: a.re - b.re,
        im: a.im - b.im,
    }
}

fn add(a: SimdComplex, b: SimdComplex)-> SimdComplex {
    SimdComplex {
        re: a.re + b.re,
        im: a.im + b.im,
    }
}

// fn inter(a: SimdComplex, b: SimdComplex) -> (SimdComplex, SimdComplex){
//     let (r0, r1) = a.re.interleave(b.re);
//     let (i0, i1) = a.im.interleave(b.im);

//     (SimdComplex{re: r0, im: i0}, SimdComplex{re: r1, im: i1})
// }


pub fn simd_fft(data: Vec<f64>) -> Vec<Complex<f64>>{
    let w: Vec<_> = (0..data.len()/2).into_par_iter()
    .map(|x| Complex::new(0.0, -2.0 * PI * x as f64 / data.len() as f64).exp())
    .collect();

    let re = simd_base(data);

    let mut tmp: Vec<_> = re.par_iter()
        .map(|&x| SimdComplex { re: x, im: f64x2::splat(0.0) })
        .collect();


    simd_rec(&mut tmp, w);

    let complex_vec: Vec<_> = tmp
        .into_par_iter()
        .flat_map(|item| {
            let re_parts: [f64; 2] = item.re.into();
            let im_parts: [f64; 2] = item.im.into();
            re_parts.iter().zip(im_parts.iter())
                .map(|(&re, &im)| Complex::new(re, im))
                .collect::<Vec<_>>()
        })
        .collect();

    return complex_vec;
}

fn simd_rec(data: &mut Vec<SimdComplex>, w: Vec<Complex<f64>>) {
    let n = data.len();
    let half = n / 2;

    if n == 4 {
        data.swap(1, 2);

        // n == 2
        let c = SimdComplex {
            re: f64x2::from_array([w[0].re(), w[2].re()]),
            im: f64x2::from_array([w[0].im(), w[2].im()]),
        };

        let c0 = mul(c, data[1]);
        let c1 = mul(c,data[3]);

        data[1] = sub(data[0], c0);
        data[0] = add(data[0], c0);
        data[3] = sub(data[2], c1);
        data[2] = add(data[2], c1);


        // n == 4
        let s0 = SimdComplex {
            re: f64x2::from_array([w[0].re(), w[1].re()]),
            im: f64x2::from_array([w[0].im(), w[1].im()]),
        };
        let s1 = SimdComplex {
            re: f64x2::from_array([w[2].re(), w[3].re()]),
            im: f64x2::from_array([w[2].im(), w[3].im()]),
        };

        let c2 = mul(s0, data[2]);
        let c3 = mul(s1, data[3]);

        data[2] = sub(data[0], c2);
        data[3] = sub(data[1], c3);
        data[0] = add(data[0], c2);
        data[1] = add(data[1], c3);

        return;
    }
    
    let mut even = Vec::with_capacity(half);
    let mut odd = Vec::with_capacity(half);
    let mut w_even = Vec::with_capacity(half);

    for i in 0..half {
        even.push(data[2 * i]);
        odd.push(data[2 * i + 1]);
        w_even.push(w[2 * i]);
    }

    simd_rec(&mut even, w_even.clone());
    simd_rec(&mut odd, w_even.clone());

    for x in 0..half {
        let c = SimdComplex{
            re: f64x2::from_array([w[2*x].re(), w[2*x + 1].re()]),
            im: f64x2::from_array([w[2*x].im(), w[2*x + 1].im()]),
        };
        let complex = mul(c, odd[x]);

        data[x] = add(even[x], complex);
        data[x + half] = sub(even[x], complex);
    }
}

// fn simd_base(data: Vec<f64>) -> Vec<Simd<f64, 2>> {
//     let size = data.len();
//     let half = size / 2;

//     // when size == 4
//     // d0 + d1, d0 - d1
//     let (left, right) = data.split_at(half);
//     let base_case:Vec<_>= left.array_chunks::<2>()
//         .map(|&left| f64x2::from_array(left))
//         .zip(right.array_chunks::<2>().map(|&right| f64x2::from_array(right)))
//         .flat_map(|(left, right)| vec![(left+right).interleave(left-right)])
//         .flat_map(|(left, right)| vec![left, right])
//         .collect();

//     // dbg!(base_case.clone());
//     return base_case;
    
//         // let interleaved_base: Vec<_> = base_case
//         // .chunks_exact(2)
//         // .flat_map(|chunk| {
//         //     let (left, right) = (chunk[0], chunk[1]);
//         //     vec![left.interleave(right)]
//         // })
//         // .flat_map(|(left, right)| vec![left,right])
//         // .collect();

//         // dbg!(interleaved_base.clone());
//         // return interleaved_base;
// }


fn simd_base(data: Vec<f64>)  -> Vec<Simd<f64, 2>> {
    let size = data.len();
    let half = size / 2;
    let base_re = Arc::new(Mutex::new(Vec::with_capacity(half)));

    // Spawn threads for interleaving the chunks concurrently
    let mut handles = vec![];
    for i in 0..half/2 {
        let left_chunk = f64x2::from_array([data[i * 2], data[i * 2 + 1]]);
        let right_chunk = f64x2::from_array([data[i * 2 + half], data[i * 2 + 1 + half]]);

        // let base_re_clone = Arc::clone(&base_re);

        let handle = std::thread::spawn(move || {
            let l = left_chunk + right_chunk;
            let r = left_chunk - right_chunk;

            let (interleaved_first, interleaved_second) = l.interleave(r);
            (interleaved_first, interleaved_second)
        });
        handles.push(handle);
    }

    // Join threads and collect results
    for handle in handles {
        let (interleaved_first, interleaved_second) = handle.join().unwrap();
        let mut base_re = base_re.lock().unwrap();
        base_re.push(interleaved_first);
        base_re.push(interleaved_second);
    }

    let tmp = Arc::try_unwrap(base_re).unwrap().into_inner().unwrap();
    // dbg!(tmp.clone());
    return tmp;

    
}

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
        // let mut p = 1;
        // let mut num_to_add = 0;  
        // loop {
        //     let a = usize::pow(2, p);
        //     if a < data.len() {
        //         p += 1;
        //     } else {
        //         num_to_add = usize::pow(2, p) - data.len();
        //         break;
        //     }
        // }
        
        // for i in 0..num_to_add{
        //     data.push(Complex::new(0.0, 0.0)); 
        // }

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
