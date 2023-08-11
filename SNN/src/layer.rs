use crate::neuron::{Neuron};
use rand::{thread_rng, Rng};

pub struct Layer{
    neurons : Vec<Neuron>,
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

}