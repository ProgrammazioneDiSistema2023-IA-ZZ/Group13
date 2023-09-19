use crate::neuron::Neuron;
use crate::errors::ConfErr;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Layer{
    pub neurons : Vec<Neuron>,
    pub range: (i32, i32)
}

impl Layer{

    pub fn new_empty(range: (i32,i32)) -> Self {
        Layer{
            neurons: Vec::new(),
            range,
        }
    }

    pub fn add_neuron(&mut self, id: i32, v_threshold: f64, v_rest: f64, v_mem: f64, v_reset: f64,funzione: fn(&mut Neuron,&Vec<u8>,&Vec<u8>,&ConfErr,i32)->u8){
        self.neurons.push( Neuron::new_without_weights(id,v_threshold,v_rest,v_mem,v_reset,funzione));
    }

    pub fn add_weights_same_layer(&mut self, id_in_layer: usize, connections_same_layer: Vec<f64>){
        self.neurons[id_in_layer].add_weights_same_layer(connections_same_layer);
    }

    pub fn add_weights_prec_layer(&mut self, id_in_layer: usize, connections_prec_layer: Vec<f64>){
        self.neurons[id_in_layer].add_weights_prec_layer(connections_prec_layer);
    }

    pub fn compute_output(&mut self, inputs_prec_layer: &Vec<u8>, inputs_same_layer: &Vec<u8>, error: &ConfErr, time: usize) -> Vec<u8>{
        let mut output = Vec::new();
        let mut i = 0;

        for n in &mut self.neurons{
            let mut inputs_same_layer_copy = inputs_same_layer.clone();
            inputs_same_layer_copy.remove(i);
            i=i+1;
            output.push(n.compute_output(&inputs_prec_layer, &inputs_same_layer_copy, error, time as i32));
        }
        output
    }

    pub fn generate_weight(n_neurons: i32, n_neurons_pre: i32, id: i32) -> (Vec<f64>, Vec<f64>){
        let mut rng = thread_rng();
        let mult = 17.0;
        let mut weights_same = Vec::new();
        for _ in 0..n_neurons-1{
            weights_same.push(  -rng.gen::<f64>()*mult/2.0 );
        }

        let mut weights_prec = Vec::new();
        if n_neurons_pre == -1{
            for j in 0..n_neurons{
                weights_prec.push( if j == id { rng.gen::<f64>()*mult }else{ 0.0 });
            }
        }
        else {
            for _ in 0..n_neurons_pre{
                weights_prec.push( rng.gen::<f64>()*mult );
            }
        }
        (weights_prec, weights_same)
    }

    pub fn id_is_in_range(&self, id: i32) -> bool{
        id >= self.range.0 && id <= self.range.1
    }
}