use std::f64::consts::E;

//to determin the activation of a neuron
//The E is the Euler constants
//E^1
pub fn sigmoid(x: f64) -> f64 {
    let sig: f64 = 1.0 / (1.0 + E.powf(-x));
    sig
}
