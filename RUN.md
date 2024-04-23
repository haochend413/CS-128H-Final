# How to run the file

## Set up the project

Make sure rust and cargo are installed. You can check with:

```bash
rustc --version
```

and

```bash
cargo --version
```

Make sure that rust is installed, clone the project from the repo:

```bash
git clone https://github.com/haochend413/CS-128H-Final.git
```

To use SIMD features for base cases, install Rust Nightly with:

```bash
rustup default nightly
```

You don't need to install nightly to use the calculator. We only implemented the SIMD features for 2\*2 base cases.

## Run the server

Run the server in VSCode terminal with:

```bash
cargo run
```

## Play in browser

After the server is running, direct to https://127.0.0.1:8080 in your local browser and type in input vector. BE SURE TO TYPE IN THE VECTOR IN THE CORRECT FORMAT!!! You can see the format on the webpage. By clicking the calculate button, the result will show on the interface. The result is shown with tuples of numbers. The first entry of every tuple demonstrates the real part of the corresponding number of the output, and the second entry deomstrates the imaginary part. To re-enter the vector, refresh the page and type in another input. Have fun!
