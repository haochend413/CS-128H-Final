# CS128H-Final-Project

### **Fast Fourier Transform**



## Group Name

**TBD**



## Members

- Bailiu Li (bailiul2)

- Binchao Ye (binchao2)

- Haochen Ding (hd9)

  

## Project Introduction

Our project focuses on implementing the Cooley-Tukey Fast Fourier Transform (FFT) algorithm in Rust, leveraging SIMD instructions and multithreading for improved performance. The Cooley-Tukey FFT is a widely used algorithm for efficiently computing the discrete Fourier transform of a sequence or its inverse. By integrating SIMD (Single Instruction, Multiple Data) instructions and multithreading, we aim to maximize parallelism and exploit hardware-level optimizations for faster FFT computation.

![DIT-FFT-butterfly](https://upload.wikimedia.org/wikipedia/commons/thumb/7/78/DIT-FFT-butterfly.svg/1920px-DIT-FFT-butterfly.svg.png)



## Technical Overview

### description

- Develop the core algorithm for computing the Cooley-Tukey FFT in Rust.

- Implement both forward and inverse FFT transformations.

- Verify the correctness of the implementation through unit tests and validation against known FFT results by Matlab.


  

### **Checkpoint 1** 

- Completion of FFT algorithm implementation.
- Integration of SIMD instructions for parallelization.
- Successful validation against test cases.

### Checkpoint 2 

- Completion of multithreading optimization.

- Comprehensive performance analysis and benchmarking.

- Finalization of documentation and project submission.

  

## Possible Challenges

- Learning the basic concepts and principles of Fast Fourier Transform
- Creating and Calculating complex numbers in Rust
- Handling SIMD optimization and multi-threading
- etc.



## References

[Cooley–Tukey FFT algorithm](https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm)

[RustFFT](https://docs.rs/rustfft/latest/rustfft/)

[SIMD | Rust by Example](https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html)

[The Fast Fourier Transform (FFT): Most Ingenious Algorithm Ever?](https://www.youtube.com/watch?v=h7apO7q16V0)

