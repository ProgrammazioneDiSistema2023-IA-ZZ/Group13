use std::fmt;
use crate::errors::{ConfErr,ErrorComponent};

#[derive(Clone)]
pub struct Neuron {
    pub id: i32,
    pub v_threshold: f64,
    pub v_rest: f64,
    pub v_mem: f64,
    pub v_reset: f64,
    pub connections_same_layer: Vec<f64>,
    pub connections_prec_layer: Vec<f64>,
    pub funzione: fn(&mut Neuron,&Vec<u8>,&Vec<u8>,&ConfErr,i32)->u8,
    pub delta_t : f64,
}


impl Neuron{

    pub fn new(id: i32, v_threshold: f64, v_rest: f64, v_mem: f64, v_reset: f64, connections_same_layer: Vec<f64>, connections_prec_layer: Vec<f64>,funzione: fn(&mut Neuron,&Vec<u8>,&Vec<u8>,&ConfErr,i32)->u8) -> Self{
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


    pub fn new_without_weights(id: i32, v_threshold: f64, v_rest: f64, v_mem: f64, v_reset: f64, funzione: fn(&mut Neuron,&Vec<u8>,&Vec<u8>,&ConfErr,i32)->u8) -> Self{
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


    pub fn compute_output(&mut self, inputs_prec_layer: &Vec<u8>, inputs_same_layer: &Vec<u8>, error: &ConfErr, time: i32) -> u8{
        if inputs_prec_layer.contains(&1) || inputs_same_layer.contains(&1) {

            (self.funzione)(self, inputs_prec_layer, inputs_same_layer, error, time)

        }else{
            self.delta_t += 1.0;
            let decrement = 0.1;
            if self.v_mem - decrement > self.v_rest{
                self.v_mem -= decrement;
            }
            0
        }
    }


    pub fn neuron_create_error(&mut self, error: &ConfErr){
        // Converte il numero in un intero e modifica il bit alla posizione desiderata
        match error.err_comp {
            ErrorComponent::Threshold => {  self.v_threshold = error.change_bit(self.v_threshold); },
            ErrorComponent::VRest => {      self.v_rest = error.change_bit(self.v_rest); },
            ErrorComponent::VMem => {       self.v_mem = error.change_bit(self.v_mem); },
            ErrorComponent::VReset => {     self.v_reset = error.change_bit(self.v_reset); },
            ErrorComponent::Weights => {
                if error.w_pos.0==0 {//prec
                    self.connections_prec_layer[error.w_pos.1] = error.change_bit(self.connections_prec_layer[error.w_pos.1]);
                }else {//same
                    self.connections_same_layer[error.w_pos.1] = error.change_bit(self.connections_same_layer[error.w_pos.1]);
                }
            }
            _ => { return;}
        }
    }
}


pub fn multiplier(x: f64, y: f64, error: &ConfErr, with_error: bool) -> f64{
    if with_error && error.err_comp == ErrorComponent::Multiplier{
        error.change_bit(x*y)
    }
    else { x*y }
}


pub fn adder(x: f64, y: f64, error: &ConfErr, with_error: bool) -> f64{
    if with_error && error.err_comp == ErrorComponent::Adder{
        error.change_bit(x+y)
    }
    else { x+y }
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

