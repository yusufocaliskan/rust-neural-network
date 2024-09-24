use std::mem::transmute;

use nn::helper;

#[derive(Debug)]
#[allow(dead_code)]
struct NeuralNetwork {
    input_layer: Vec<f64>,
    hidden_layer: Vec<f64>,
    output_layer: Vec<f64>,
    weights_ih: Vec<Vec<f64>>,
    weights_ho: Vec<Vec<f64>>,
}

impl NeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let input_layer = vec![0.0; input_size];
        let hidden_layer = vec![0.0; hidden_size];
        let output_layer = vec![0.0; output_size];

        //the weights between input and hidden layers
        let weights_ih = vec![vec![0.5; hidden_size]; input_size];

        //hidden output
        let weights_ho = vec![vec![0.5; output_size]; hidden_size];

        NeuralNetwork {
            input_layer,
            hidden_layer,
            output_layer,
            weights_ih,
            weights_ho,
        }
    }

    //nuerons output
    fn feedforward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        self.input_layer = inputs;

        //calculate layer outs
        //create hidden layes
        for i in 0..self.hidden_layer.len() {
            let mut sum = 0.0;

            //let's sum inputs with weights
            for j in 0..self.input_layer.len() {
                sum += self.input_layer[j] * self.weights_ih[j][i];
            }

            self.hidden_layer[i] = helper::math::sigmoid(sum);

            // println!("{}", sum);
            // println!("{}", self.hidden_layer[i]);
        }

        //okay lets calculte output layer result
        for i in 0..self.output_layer.len() {
            let mut sum = 0.0;
            for j in 0..self.hidden_layer.len() {
                sum += self.hidden_layer[j] * self.weights_ho[j][i];
            }

            self.output_layer[i] = helper::math::sigmoid(sum);
        }

        //set the output layer. will be use in training
        self.output_layer.clone()
    }

    //To train the nn
    //we will try to use backpropagation algriothm approach
    /// * `input` - training date
    /// * `target` - the result we expecting
    /// * `learning_rate` -  learning_speed
    fn train(&mut self, inputs: Vec<f64>, targets: Vec<f64>, learning_rate: f64) {
        //step#1. Error for output layer (formula: target-output)
        //step#2. Error of the Hidden layer (formulla: output error * weighth)
        //step#3. Re calculate (update) weighth (of output layer)

        //step#1
        //differentiation betwen output and target
        //using to calculate weighths
        let outputs = self.feedforward(inputs);
        let mut output_errors = vec![0.0; self.output_layer.len()];
        for i in 0..self.output_layer.len() {
            output_errors[0] = targets[i] - outputs[i];
        }

        println!("output_errors--> {:#?}", output_errors);

        //step#2
        let mut hidden_errors = vec![0.0; self.hidden_layer.len()];

        for i in 0..self.hidden_layer.len() {
            let mut error = 0.0;

            for j in 0..self.output_layer.len() {
                error += output_errors[j] * self.weights_ho[i][j];
            }

            hidden_errors[i] = error
        }
    }
}

fn main() {
    //data for XOR
    let training_data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];
    //2 inputs
    //2 hidden layers
    //1 output

    let mut nn = NeuralNetwork::new(2, 2, 1);

    for _ in 0..10 {
        for &(ref inputs, ref targets) in &training_data {
            nn.train(inputs.clone(), targets.clone(), 0.1);
        }
    }

    println!("[0,0] --> {:#?}", nn);
    // nn.feedforward(vec![0.0, 1.0]);
    // println!("[0,1] --> {:#?}", nn);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nn_test_initializer() {
        //2 inputs
        //2 hidden layers
        //1 output
        let mut nn = NeuralNetwork::new(2, 2, 1);
        nn.feedforward(vec![0.0, 0.0]);
        nn.feedforward(vec![0.0, 1.0]);
    }
}
