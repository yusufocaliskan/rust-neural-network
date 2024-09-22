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
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nn_test_initializer() {
        let nn = NeuralNetwork::new(4, 4, 4);
        println!("{:#?}", nn);
    }
}
