# Neural Network with Rust

This project implements a simple neural network in Rust to solve the XOR problem. The neural network is capable of training itself using the backpropagation algorithm.

Use the fallowing command to get the static result

```shell
cargo run
```

XOR Test Result:

```shell
cargo test xor_test_nn

//to display the result
cargo test xor_test_nn -- --nocapture
```

```shell
    Test XOR [0-0] -> 0.07447127026483495
    Test XOR [0-1] -> 0.9442157962753881
    Test XOR [1-0] -> 0.9427510736015517
    Test XOR [1-0] -> 0.047543707694473566
```

AND

```shell
cargo test and_test_nn


//to display the result
cargo test and_test_nn -- --nocapture
```

```shell
Question for And --> 0-0 : [
    0.006176246532957251,]
Question for And --> 0-1 : [
    0.02496336563462987, ]
Question for And --> 1-0 : [
    0.023030271958667296,]
Question for And --> 1-1 : [
    0.966386645374941,
]

```

NAND

```shell
cargo test nand_test_nn


//to display the result
cargo test nand_test_nn -- --nocapture
```

```shell

Question for NAnd --> 0-0 : [
    0.9938314990590297,]
Question for NAnd --> 0-1 : [
    0.9796065695507552,]
Question for NAnd --> 1-0 : [
    0.9777487513661111,]
Question for NAnd --> 1-1 : [
    0.02887171712103688,]
```
