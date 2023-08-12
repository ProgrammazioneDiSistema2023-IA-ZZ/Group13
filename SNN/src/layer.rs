use crate::neuron::{Neuron};
use rand::{thread_rng, Rng};

pub struct Layer{
    pub neurons : Vec<Neuron>,
    input : Vec<i32>,
    output : Vec<i32>,
    output_prec : Vec<i32>,
    // sender : Sender<Vec<i32>>,
    // receiver : Receiver<Vec<i32>>,
}

impl Layer{

    pub fn new(n_neurons : i32, n_neurons_pre : i32) -> Self {
        let mut rng = thread_rng();
        let mut neurons = Vec::new();

        for i in 0..n_neurons{
            let mut weights_same = Vec::new();
            for j in 0..n_neurons{
                weights_same.push(  rng.gen::<f64>() );
            }

            let mut weights_prec = Vec::new();
            for j in 0..n_neurons_pre{
                weights_prec.push(  rng.gen::<f64>() );
            }

            neurons.push( Neuron::new(1.0,1.0,1.0,1.0,weights_same, weights_prec) )

        }

        let mut input = Vec::new();
        let mut output = Vec::new();
        let mut output_prec = Vec::new();
        Layer{
            neurons,
            input,
            output,
            output_prec,
        }

    }

    pub fn compute_output(&mut self, inputs_prec_layer : &Vec<i32>, inputs_same_layer : &Vec<i32>) -> Vec<i32>{
        let mut output = Vec::new();
        for mut n in self.neurons{
            output.push(n.compute_output(&inputs_prec_layer, inputs_same_layer) );
        }
        output
    }




}