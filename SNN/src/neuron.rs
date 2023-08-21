use std::fmt;
use crate::errors::{ConfErr,ErrorComponent,Type};

#[derive(Clone)]
pub struct Neuron {
    pub id: i32,
    pub v_threshold: f64,
    pub v_rest: f64,
    pub v_mem: f64, //la struct dovrà essere mutabile cosicchè ogni volta v_mem cambia in base al t
    pub v_reset: f64,
    pub connections_same_layer: Vec<f64>,
    pub connections_prec_layer: Vec<f64>,
    pub funzione: fn(&mut Neuron,&Vec<i32>,&Vec<i32>)->i32,
    pub delta_t : f64,
}


impl Neuron{

    pub fn new(id: i32, v_threshold: f64, v_rest: f64, v_mem: f64, v_reset: f64, connections_same_layer: Vec<f64>, connections_prec_layer: Vec<f64>,funzione: fn(&mut Neuron,&Vec<i32>,&Vec<i32>)->i32) -> Self{
        let delta_t = 1.0;

        Neuron {
            id,
            v_threshold,
            v_rest,
            v_mem,
            v_reset,
            connections_same_layer,
            connections_prec_layer,
            funzione,
            delta_t
        }
    }

    pub fn new_without_weights(id: i32, v_threshold: f64, v_rest: f64, v_mem: f64, v_reset: f64,funzione: fn(&mut Neuron,&Vec<i32>,&Vec<i32>)->i32) -> Self{
        let connections_same_layer = vec![];
        let connections_prec_layer = vec![];
        let delta_t = 1.0;

        Neuron {
            id,
            v_threshold,
            v_rest,
            v_mem,
            v_reset,
            connections_same_layer,
            connections_prec_layer,
            funzione,
            delta_t
        }
    }

    pub fn add_weights_same_layer(&mut self, connections_same_layer: Vec<f64>){
        self.connections_same_layer = connections_same_layer;
    }
    pub fn add_weights_prec_layer(&mut self, connections_prec_layer: Vec<f64>){
        self.connections_prec_layer = connections_prec_layer;
    }

    /*pub fn compute_output(&mut self, inputs_prec_layer: &Vec<i32>, inputs_same_layer: &Vec<i32>, layer_errors: &mut Vec<ConfErr>, time: i32) -> i32{ //sarà chiamata dalla rete grande
        let decrement = 0.1;
        if inputs_prec_layer.contains(&1) || inputs_same_layer.contains(&1) {
            for neuron_error in layer_errors{
                if neuron_error.id_neuron == self.id && neuron_error.t_start <= time && neuron_error.t_start+neuron_error.duration >= time {
                    //println!("Neurone: {}, time: {}, before error: {}, original_parameter: {}, tupla: {:?}",self.id, time, self.v_threshold, error.original_parameter, error.w_pos);
                    self.neuron_create_error(neuron_error, time);
                    //println!("prova di salvataggio original: {}", error.original_parameter);
                    //println!("after error: {}",self.v_threshold);
                }
            }

            return (self.funzione)(self, inputs_prec_layer, inputs_same_layer);
        }
        self.delta_t += 1.0;
        if self.v_mem - decrement > self.v_rest{
            self.v_mem -= decrement;
        }
        0
    }*/

    pub fn compute_output(&mut self, inputs_prec_layer: &Vec<i32>, inputs_same_layer: &Vec<i32>, error: &ConfErr, time: i32) -> i32{ //sarà chiamata dalla rete grande
        let decrement = 0.1;
        if inputs_prec_layer.contains(&1) || inputs_same_layer.contains(&1) {
            if error.id_neuron == self.id && ((error.err_type == Type::BitFlip && error.t_start == time) || (error.err_type == Type::Stuck0 || error.err_type == Type::Stuck1) ){
                //println!("Neurone: {}, time: {}, before error: {}, original_parameter: {}, tupla: {:?}",self.id, time, self.v_threshold, error.original_parameter, error.w_pos);
                self.neuron_create_error(error);
                //println!("prova di salvataggio original: {}", error.original_parameter);
                //println!("after error: {}",self.v_threshold);
            }

            return (self.funzione)(self, inputs_prec_layer, inputs_same_layer);
        }
        self.delta_t += 1.0;
        if self.v_mem - decrement > self.v_rest{
            self.v_mem -= decrement;
        }
        0
    }

    fn neuron_create_error(&mut self, error: &ConfErr){
        let mut number;
        let bit_position = error.n_bit; // Posizione del bit da modificare
        // Converte il numero in un intero e modifica il bit alla posizione desiderata
        match error.err_comp {
            ErrorComponent::Threshold => { number = self.v_threshold; },
            ErrorComponent::VRest => { number = self.v_rest; },
            ErrorComponent::VMem => { number = self.v_mem; },
            ErrorComponent::VReset => { number = self.v_reset; },
            ErrorComponent::Weights => {
                if error.w_pos.0==0 {//prec
                    number = self.connections_prec_layer[error.w_pos.1];
                }else {//same
                    number = self.connections_same_layer[error.w_pos.1];
                }
            }
        }

        let mut bits: u64 = number.to_bits();
        match error.err_type {
            Type::Stuck0 => {
                let mask = !(1 << bit_position);
                bits &= mask;// stuck ad 0
            },
            Type::Stuck1 => {
                let mask = 1 << bit_position;
                bits &= mask;// stuck ad 1
            },
            Type::BitFlip => {
                bits ^= 1 << bit_position; // Esegue un XOR per invertire il bit
            }
            Type::None => {
                panic!("impossible, NoError here!")
            }
        }

        // Converte nuovamente gli "bits di floating point" in un f64 modificato
        number = f64::from_bits(bits);


        match error.err_comp {
            ErrorComponent::Threshold => { /*println!("threshold {}, Modified number: {}", self.v_threshold, number);*/ self.v_threshold = number;  },
            ErrorComponent::VRest => { /*println!("v_mem {}, Modified number: {}", self.v_mem, number);*/ self.v_rest = number; },
            ErrorComponent::VMem => { /*println!("v_mem {}, Modified number: {}", self.v_mem, number);*/ self.v_mem = number; },
            ErrorComponent::VReset => { /*println!("v_mem {}, Modified number: {}", self.v_mem, number);*/ self.v_reset = number; },
            ErrorComponent::Weights => {
                if error.w_pos.0==0 {//prec
                    self.connections_prec_layer[error.w_pos.1] = number;
                }else {//same
                    self.connections_same_layer[error.w_pos.1] = number;
                }
            }
        }
    }
}



impl fmt::Display for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut s1 = "[ ".to_owned();
        for i in &self.connections_same_layer{
            // s1 = s1 + &round_f64(i).to_string();
            s1 = s1 +  format!("{:.2}", i).as_str() ;

            s1 = s1 + ", ";
        }
        if s1.len()>2{
            s1.pop();
            s1.pop();
        }
        s1 = s1 + " ]";

        let mut s2 = "[ ".to_owned();
        for i in &self.connections_prec_layer{
            // s1 = s1 + &round_f64(i).to_string();
            s2 = s2 +  format!("{:.2}", i).as_str() ;

            s2 = s2 + ", ";
        }
        if s2.len()>2{
            s2.pop();
            s2.pop();
        }

        s2 = s2 + " ]";

        write!(f, "Neuron : id : {}, v_threshold : {}, v_rest : {}, v_mem  : {}, v_reset : {}, connections_same_layer : {}, connections_prec_layer : {}",
               self.id,
               round_f64(self.v_threshold),
               round_f64(self.v_rest),
               round_f64(self.v_mem),
               round_f64(self.v_reset),
                s1, s2)
    }
}

pub fn round_f64(n : f64) -> f64{
    (n * 100.0).round() / 100.0
}

