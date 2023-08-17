use crate::neuron::Neuron;
use crate::errors::ConfErr;
use crate::errors::ErrorComponent;
use crate::errors::Type;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Layer{
    pub neurons : Vec<Neuron>,
    range: (i32, i32)
    // input : Vec<i32>,
    // output : Vec<i32>,
    // output_prec : Vec<i32>,
}

impl Layer{

    pub fn new(start_id: i32, n_neurons: i32, n_neurons_pre: i32) -> Self {
        let mut rng = thread_rng();
        let mut neurons = Vec::new();
        let mut id = start_id;
        let mult = 15.0;

        for _ in 0..n_neurons{
            let mut weights_same = Vec::new();
            for _ in 0..n_neurons-1{
                weights_same.push(  rng.gen::<f64>()*mult );
            }

            let mut weights_prec = Vec::new();
            if n_neurons_pre == -1{
                for j in 0..n_neurons{
                    weights_prec.push(   if j == id { rng.gen::<f64>()*mult }else{ 0.0 });
                }
            }
            else {
                for _ in 0..n_neurons_pre{
                    weights_prec.push( rng.gen::<f64>()*mult );
                }
            }

            neurons.push( Neuron::new(id,-52.0,-65.0,-65.0,-60.0,weights_same, weights_prec) );
            id = id + 1;
        }

        Layer{
            neurons,
            range: (start_id, start_id+n_neurons-1)
        }

    }

    pub fn compute_output(&mut self, inputs_prec_layer: &Vec<i32>, inputs_same_layer: &Vec<i32>, errors_vec: &mut Vec<ConfErr>, time: usize) -> Vec<i32>{
        let mut output = Vec::new();
        let mut i = 0;

        for n in &mut self.neurons{
            let mut inputs_same_layer_copy = inputs_same_layer.clone();
            inputs_same_layer_copy.remove(i);
            i=i+1;
            output.push(n.compute_output(&inputs_prec_layer, &inputs_same_layer_copy, errors_vec, time as i32));
        }
        output
    }

    pub fn create_vec_err(&self, type_err: Type, n_errors: i32, tot_time: i32) -> Vec<ConfErr>{
        if n_errors <= 0{
            return vec![];
        }

        let mut rng = thread_rng();
        let mut errors_vec: Vec<ConfErr> = Vec::new();
        for _ in 0..n_errors{
            'loop_flag: loop{
                let id_n = rng.gen_range(self.range.0..=self.range.1);
                let t_start = rng.gen_range(0..tot_time);
                let duration;
                if let Type::BitFlip=type_err { duration=1 }
                else {duration = rng.gen_range(1..=3/*(tot_time-t_start)*/); }

                for e in &errors_vec{
                    if e.id_neuron == id_n && e.is_overlapping(t_start,duration){
                        continue 'loop_flag;
                    }
                }

                let bit = rng.gen_range(0..64);
                let flag = rng.gen_range(0..3);
                let cmpn;
                match flag {
                    0 => cmpn = ErrorComponent::Threshold,
                    1 => cmpn = ErrorComponent::VMem,
                    2 => cmpn = ErrorComponent::Weights,
                    _ => panic!("impossible")
                }

                let err = ConfErr::new(id_n,t_start,duration,bit,type_err,cmpn,0.0, (0,0));
                errors_vec.push(err);
                break 'loop_flag;
            }
        }
        println!("creati errori: {:?}",errors_vec);
        errors_vec
    }
    /*pub fn clone_neuron(&self)-> Neuron{
        self.neurons[0].clone()
    }*/



}