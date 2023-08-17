use rand::{thread_rng, Rng};
use std::fmt;
use crate::errors::{ConfErr,ErrorComponent,Type};

//#[derive(Debug)]
// pub struct Connection {
//     id_input: i32,
//     weight: f64
// }
//
// impl Connection {
//
//     pub fn new(id_input:i32, weight:f64) -> Self{
//         Connection{
//             id_input,
//             weight
//         }
//     }
// }


#[derive(Debug, Clone)]
pub struct Neuron {
    id: i32,
    v_threshold: f64,
    v_rest: f64,
    v_mem: f64, //la struct dovrà essere mutabile cosicchè ogni volta v_mem cambia in base al t
    v_reset: f64,
    connections_same_layer: Vec<f64>,
    connections_prec_layer: Vec<f64>
}

impl Neuron{

    pub fn new( id: i32, v_threshold: f64, v_rest: f64, v_mem: f64, v_reset: f64, connections_same_layer: Vec<f64>, connections_prec_layer: Vec<f64>) -> Self{

        Neuron {
            id,
            v_threshold,
            v_rest,
            v_mem, //valore t0
            v_reset,
            connections_same_layer,
            connections_prec_layer,
        }
    }



    pub fn compute_output(&mut self, inputs_prec_layer: &Vec<i32>, inputs_same_layer: &Vec<i32>, layer_errors: &mut Vec<ConfErr>, time: i32) -> i32{ //sarà chiamata dalla rete grande
        for neuron_error in layer_errors{
            if neuron_error.id_neuron == self.id && neuron_error.t_start <= time && neuron_error.t_start+neuron_error.duration >= time {
                //println!("Neurone: {}, time: {}, before error: {}, original_parameter: {}, tupla: {:?}",self.id, time, self.v_threshold, error.original_parameter, error.w_pos);
                self.neuron_create_error(neuron_error, time);
                //println!("prova di salvataggio original: {}", error.original_parameter);
                //println!("after error: {}",self.v_threshold);
            }
        }
        self.v_mem = self.v_rest + (self.v_mem - self.v_rest)*f64::exp(-1.0/0.1);
        //let _v_m = self.v_mem.clone();

        for i in 0..inputs_prec_layer.len(){
            self.v_mem += inputs_prec_layer[i] as f64 * self.connections_prec_layer[i];

        }

       for i in 0..inputs_same_layer.len(){
           self.v_mem += inputs_same_layer[i] as f64 * self.connections_same_layer[i];
        }

        // println!("id : {}, v_mem : {} -> {}", self.id, v_m, self.v_mem);
        if self.v_mem > self.v_threshold{
            self.v_mem = self.v_reset;
            return 1;
        }
        0
    }



    fn neuron_create_error(&mut self, error: &mut ConfErr, time: i32){
        let mut rng = thread_rng();
        let mut number;
        let bit_position = error.n_bit; // Posizione del bit da modificare
        let mut ending = false;
        // Converte il numero in un intero e modifica il bit alla posizione desiderata
        match error.err_comp {
            ErrorComponent::Threshold => {
                if error.t_start==time { error.original_parameter = self.v_threshold; }
                if error.t_start+error.duration==time { self.v_threshold=error.original_parameter; ending=true; }
                number = self.v_threshold; },
            ErrorComponent::VMem => {
                if error.t_start==time { error.original_parameter = self.v_mem; }
                if error.t_start+error.duration==time { self.v_mem=error.original_parameter; ending=true; }
                number = self.v_mem; },
            ErrorComponent::Weights => {
                if error.t_start==time {
                    let vec = rng.gen_range(0..2);
                    let len;
                    let index;
                    if vec==0 {//prec
                        len = self.connections_prec_layer.len();
                        index = rng.gen_range(0..len) as usize;
                        error.original_parameter = self.connections_prec_layer[index];
                    }else {//same
                        len = self.connections_same_layer.len();
                        index = rng.gen_range(0..len) as usize;
                        error.original_parameter = self.connections_same_layer[index];
                    }

                    error.w_pos = (vec, index);
                }
                if error.t_start+error.duration==time {
                    if error.w_pos.0==0 {//prec
                        self.connections_prec_layer[error.w_pos.1]=error.original_parameter;
                    }else {//same
                        self.connections_same_layer[error.w_pos.1]=error.original_parameter;
                    }
                    ending=true;
                }
                if error.w_pos.0==0 {//prec
                    number = self.connections_prec_layer[error.w_pos.1];
                }else {//same
                    number = self.connections_same_layer[error.w_pos.1];
                }
            }
        }

        if ending==true{
            return;
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
        }

        // Converte nuovamente gli "bits di floating point" in un f64 modificato
        number = f64::from_bits(bits);


        match error.err_comp {
            ErrorComponent::Threshold => { /*println!("threshold {}, Modified number: {}", self.v_threshold, number);*/ self.v_threshold = number;  },
            ErrorComponent::VMem => { /*println!("v_mem {}, Modified number: {}", self.v_mem, number);*/ self.v_mem = number; },
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
        s1.pop();
        s1.pop();
        s1 = s1 + " ]";

        let mut s2 = "[ ".to_owned();
        for i in &self.connections_prec_layer{
            // s1 = s1 + &round_f64(i).to_string();
            s2 = s2 +  format!("{:.2}", i).as_str() ;

            s2 = s2 + ", ";
        }
        s2.pop();
        s2.pop();
        s2 = s2 + " ]";

        write!(f, "Neuron : id : {}, v_rest : {}, v_threshold : {}, v_mem  : {}, v_reset : {}, connections_same_layer : {}, connections_prec_layer : {}",
               self.id,
               round_f64(self.v_rest),
               round_f64(self.v_threshold),
               round_f64(self.v_mem),
               round_f64(self.v_reset),
                s1, s2)
    }
}

pub fn round_f64(n : f64) -> f64{
    (n * 100.0).round() / 100.0
}

