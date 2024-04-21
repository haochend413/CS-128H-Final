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

## Run the server

Run the server in VSCode terminal with:

```bash
cargo run
```

## Play in browser

Direct to https://127.0.0.1:8080 in local browser and type in input vector. By clicking the calculate button, the result will show on the interface. To re-enter the vector, refresh the page and type in another input. Have fun!
