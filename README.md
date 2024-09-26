# Neural Network with Rust

This project implements a simple neural network in Rust to solve the XOR problem. The neural network is capable of training itself using the backpropagation algorithm.

Use the fallowing command to get the static result

```shell
cargo run
```

Test Result:

```shell
    Question --> 0-0 --> 0 : [ 0.05498023387754637 ]
    Question --> 0-1 --> 1 : [ 0.9472457448290018  ]
    Question --> 1-0 --> 1 : [ 0.9453824744560335  ]
    Question --> 1-1 --> 0 : [ 0.0518002153001278  ]
```

To run the test

```shell
cargo tset test_nn
```
