fn main() {
    println!("Hello, world!");
}

extern crate num;
//read https://docs.rs/num/latest/num/complex/struct.Complex.html !!
use std::f64::consts::PI;
use num::complex::Complex;

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
/// let output = fft::fft(vec![5.0, 3.0, 2.0, 1.0]);
/// assert_eq!(output[0], Complex::new(11.0, 0.0));
/// assert_eq!(output[1], Complex::new(3.0, -2.0));
/// assert_eq!(output[2], Complex::new(3.0, 0.0));
/// assert_eq!(output[3], Complex::new(3.0, 2.0));
/// ```
pub fn fft(input: Vec<f64>) -> Vec<Complex<f64>> {
    let temp = FastFourierTransform::new(input);
    let mut vec = temp.input_vector.clone();
    temp.fft_rec(&mut vec);
    vec
}

//need a better name
pub struct FastFourierTransform {
    pub input_vector: Vec<Complex<f64>>,
    pub complex_vector: Vec<Complex<f64>>,
    pub size: usize,
}

impl FastFourierTransform {
    //note: input_vector.re = data, input_vector.im = 0.0;
    //calculate w = e^(1i*2*PI*data[index]/data.len()), then store it into complex_vector
    pub fn new(data:Vec<f64>) -> FastFourierTransform {
        todo!()
    }

    

    //Splite the input_vector input into even array and odd array, then recursively call fft_rec() until hit base case: N == 2,
    //then compute the basic size 2 DFT butterfly and return, and After that, combining the value at each level
    //WHAT IF: data.len() is not power of 2??? (IDK)
    pub fn fft_rec(&self, data: &mut Vec<Complex<f64>>){
        let n = data.len();
        if  n == 2{
            //data[0] = data[0] + data[1]
            //data[1] = data[0] - data[1]
            return;
        }
        //split input into even array and odd array, then recursive call fft_rec() * 2 here

        //----------------------------------------------------------------------------------//
        //for x in 0..n/2
        //index = N/n * x, where N = self.size, and n = data.len() 
        //complex = self.complex_vector[index] * data_odd[index]
        //data[x] = data_even[x] + complex
        //data[x + n/2] = data_even[x] - complex

    }
}
