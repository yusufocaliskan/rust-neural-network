use nn::helper;
use rand::Rng;

#[derive(Debug)]
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

        println!("----- Start ----");
        helper::debug::pr("input_layer", &input_layer);
        helper::debug::pr("input_layer", &input_layer);
        helper::debug::pr("output_layer", &output_layer);

        //random
        let mut rng = rand::thread_rng();

        //the weights between input and hidden layers

        // let weights_ih = vec![vec![0.2; hidden_size]; input_size];
        let weights_ih = (0..input_size)
            .map(|_| (0..hidden_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();

        //hidden output
        // let weights_ho = vec![vec![0.2; output_size]; hidden_size];
        let weights_ho = (0..hidden_size)
            .map(|_| (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();

        // println!("weights_ih-- > {:#?}", weights_ih);
        // println!("weights_ho-- > {:#?}", weights_ho);

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

            //let's out sum inputs with weights
            for j in 0..self.input_layer.len() {
                sum += self.input_layer[j] * self.weights_ih[j][i];
            }

            //Try relu in the hidden  layer
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

            //we deiced if tricks the neuron
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
        // println!("Outputs of the Neurons --> {:#?}", outputs);

        let mut output_errors = vec![0.0; self.output_layer.len()];
        for i in 0..self.output_layer.len() {
            output_errors[i] = targets[i] - outputs[i];
        }

        // println!("output_errors--> {:#?}", output_errors);

        //step#2
        let mut hidden_errors = vec![0.0; self.hidden_layer.len()];

        for i in 0..self.hidden_layer.len() {
            let mut error = 0.0;

            for j in 0..self.output_layer.len() {
                error += output_errors[j] * self.weights_ho[i][j];
            }

            hidden_errors[i] = error
        }

        for i in 0..self.output_layer.len() {
            for j in 0..self.hidden_layer.len() {
                //We need to calculate
                //effects of the errors to the weighths
                let delta =
                    output_errors[i] * helper::math::sigmoid_derivative(self.output_layer[i]);

                self.weights_ho[j][i] += learning_rate * delta * self.hidden_layer[j];
            }
        }

        for i in 0..self.hidden_layer.len() {
            for j in 0..self.input_layer.len() {
                let delta =
                    hidden_errors[i] * helper::math::sigmoid_derivative(self.hidden_layer[i]);
                self.weights_ih[j][i] += learning_rate * delta * self.input_layer[j];
            }
        }
    }

    fn perform_training(&mut self, training_data: Vec<(Vec<f64>, Vec<f64>)>, epochs: usize) {
        for _ in 0..epochs {
            for &(ref inputs, ref targets) in &training_data {
                // println!("Inputs {:#?}", inputs.clone());
                // println!("Target--> {:#?}", targets.clone());
                self.train(inputs.clone(), targets.clone(), 0.1);
            }
        }
    }

    fn debug(&self, text: &str) {
        helper::debug::pr(text, &self);
    }
}

fn main() {
    //data for XOR
    //2 inputs
    //2 hidden layers
    //1 output
    let training_data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    let mut nn = NeuralNetwork::new(2, 10, 1);

    nn.debug("---Before feeding  ---");
    nn.perform_training(training_data, 5000);
    nn.debug("---After feeding ---");

    println!("Question  --> 0-0 : {:#?}", nn.feedforward(vec![0.0, 0.0]));
    println!("Question --> 0-1 : {:#?}", nn.feedforward(vec![0.0, 1.0]));
    println!("Question --> 1-0 : {:#?}", nn.feedforward(vec![1.0, 0.0]));
    println!("Question --> 1-1 : {:#?}", nn.feedforward(vec![1.0, 1.0]));
}

#[cfg(test)]
mod test {
    use super::*;

    //XOR
    #[test]
    fn xor_test_nn() {
        //2 inputs
        //2 hidden layers
        //1 output

        let mut nn = NeuralNetwork::new(2, 10, 1);

        let training_data = vec![
            (vec![0.0, 0.0], vec![0.0]),
            (vec![0.0, 1.0], vec![1.0]),
            (vec![1.0, 0.0], vec![1.0]),
            (vec![1.0, 1.0], vec![0.0]),
        ];

        //for better approach check the main() fn
        for _ in 0..5000 {
            for &(ref inputs, ref targets) in &training_data {
                // println!("Inputs {:#?}", inputs.clone());
                // println!("Target--> {:#?}", targets.clone());
                nn.train(inputs.clone(), targets.clone(), 0.1);
            }
        }
        // println!("Question --> 0-1 : {:#?}", nn.feedforward(vec![0.0, 1.0]));
        // println!("Question --> 1-0 : {:#?}", nn.feedforward(vec![1.0, 0.0]));
        // println!("Question --> 1-1 : {:#?}", nn.feedforward(vec![1.0, 1.0]));
        // println!("NeuralNetwork --> Result {:#?}", nn);

        let tolerance = 0.09;

        let output_00 = nn.feedforward(vec![0.0, 0.0])[0];
        println!("Test XOR [0-0] -> {:?}", output_00);
        assert!(
            helper::debug::debug_helper(output_00, 0.0, tolerance),
            "Failed--> [0-0]"
        );

        let output_01 = nn.feedforward(vec![0.0, 1.0])[0];
        println!("Test XOR [0-1] -> {:?}", output_01);
        assert!(
            helper::debug::debug_helper(output_01, 1.0, tolerance),
            "Failed--> [0-1]"
        );

        let output_10 = nn.feedforward(vec![1.0, 0.0])[0];
        println!("Test XOR [1-0] -> {:?}", output_10);
        assert!(
            helper::debug::debug_helper(output_10, 1.0, tolerance),
            "Failed--> [1-0]"
        );

        let output_11 = nn.feedforward(vec![1.0, 1.0])[0];
        println!("Test XOR [1-0] -> {:?}", output_11);
        assert!(
            helper::debug::debug_helper(output_11, 0.0, tolerance),
            "Failed--> [1-0]"
        );
    }

    //AND
    #[test]
    fn and_test_nn() {
        let and_training_data = vec![
            (vec![0.0, 0.0], vec![0.0]),
            (vec![0.0, 1.0], vec![0.0]),
            (vec![1.0, 0.0], vec![0.0]),
            (vec![1.0, 1.0], vec![1.0]),
        ];

        let mut nn = NeuralNetwork::new(2, 10, 1);
        nn.perform_training(and_training_data, 5000);

        println!("Test AND --> 0-0 : {:#?}", nn.feedforward(vec![0.0, 0.0]));
        println!("Test AND --> 0-1 : {:#?}", nn.feedforward(vec![0.0, 1.0]));
        println!("Test AND --> 1-0 : {:#?}", nn.feedforward(vec![1.0, 0.0]));
        println!("Test AND --> 1-1 : {:#?}", nn.feedforward(vec![1.0, 1.0]));
    }

    //NAND
    #[test]
    fn nand_test_nn() {
        let and_training_data = vec![
            (vec![0.0, 0.0], vec![1.0]),
            (vec![0.0, 1.0], vec![1.0]),
            (vec![1.0, 0.0], vec![1.0]),
            (vec![1.0, 1.0], vec![0.0]),
        ];

        let mut nn = NeuralNetwork::new(2, 10, 1);
        nn.perform_training(and_training_data, 5000);

        println!("Test NAND --> 0-0 : {:#?}", nn.feedforward(vec![0.0, 0.0]));
        println!("Test NAND --> 0-1 : {:#?}", nn.feedforward(vec![0.0, 1.0]));
        println!("Test NAND --> 1-0 : {:#?}", nn.feedforward(vec![1.0, 0.0]));
        println!("Test NAND --> 1-1 : {:#?}", nn.feedforward(vec![1.0, 1.0]));
    }
}
