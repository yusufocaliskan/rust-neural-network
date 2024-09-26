# Neural Network with Rust

This project implements a simple neural network in Rust to solve the XOR problem. The neural network is capable of training itself using the backpropagation algorithm.

Use the fallowing command to get the static result

```shell
cargo run
```

XOR Test Result:

```shell
cargo tset test_nn
```

```shell
    Question --> 0-0 --> 0 : [ 0.05498023387754637 ]
    Question --> 0-1 --> 1 : [ 0.9472457448290018  ]
    Question --> 1-0 --> 1 : [ 0.9453824744560335  ]
    Question --> 1-1 --> 0 : [ 0.0518002153001278  ]
```

```shell
cargo tset test_nn_and
```

AND

```shell
Question for And --> 0-0 : [
    0.006176246532957251,
]
Question for And --> 0-1 : [
    0.02496336563462987,
]
Question for And --> 1-0 : [
    0.023030271958667296,
]
Question for And --> 1-1 : [
    0.966386645374941,
]

```

NAND

```shell
cargo tset test_nn_nand
```

```shell

Question for NAnd --> 0-0 : [
    0.9938314990590297,
]
Question for NAnd --> 0-1 : [
    0.9796065695507552,
]
Question for NAnd --> 1-0 : [
    0.9777487513661111,
]
Question for NAnd --> 1-1 : [
    0.02887171712103688,
]
test test::test_nn_nand ... ok
```
