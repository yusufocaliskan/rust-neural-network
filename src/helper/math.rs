use std::f64::consts::E;

//to determin the activation of a neuron
//The E is the Euler constants
//E^1
pub fn sigmoid(x: f64) -> f64 {
    let sig: f64 = 1.0 / (1.0 + E.powf(-x));
    sig
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    x * (1.0 - x)
}

pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

pub fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_sig() {
//         let s = sigmoid(1.0);
//     }

//     #[test]
//     fn test_sig_derivative() {
//         let sd = sigmoid_derivative(0.5);
//     }
// }
