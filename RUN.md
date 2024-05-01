# How to run the project

## Set up the project

Make sure rust and cargo are installed on your system. You can check with:

```bash
rustc --version
```

and

```bash
cargo --version
```

For rust and cargo installation guide, please visit https://doc.rust-lang.org/cargo/getting-started/installation.html; 

Clone the project from the repo:

```bash
git clone https://github.com/haochend413/FFT-Calculator.git
```

To use SIMD features for base cases, install Rust Nightly with:

```bash
rustup default nightly
```

You don't need to install nightly to use the calculator. We only implemented the SIMD features for 2\*2 base cases.

## Run the server
### General Procedure
Direct into the directory and run the server in VSCode terminal with:

```bash
cd FFT-Calculator
```

```bash
cargo run
```
### On Windows
Please run our project on linux. To run the project on Windows, you might consider install the stable Rust toolchain for the 64-bit Windows GNU target (x86_64-pc-windows-gnu) using rustup, and install GCC on Windows. 

## Play in browser

After the server is running, direct to https://127.0.0.1:8080 in your local browser and type in the input vector. If this doesn't work, please try copying 127.0.0.1:8080 into the search bar of your browser and enter. 

**BE SURE TO TYPE IN THE VECTOR IN THE CORRECT FORMAT!!!** You can see the format on the webpage. To be more specific, be sure that your vector is comma separated and **there are extra commas on both ends of the vector**. 
For example, if you want to enter 1,1,1,1 as your input, please type in: 

```bash
,1,1,1,1,
```

By clicking the calculate button, the result will show on the interface. The result is shown with tuples of numbers, with the first entry of every tuple demonstrates the real part of the corresponding number of the output, and the second entry demonstrates the imaginary part. To re-enter a vector, refresh the page and type in another input. Have fun!
