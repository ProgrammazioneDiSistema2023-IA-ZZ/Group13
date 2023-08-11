extern crate rand;
use std::fmt;
use rand::{thread_rng, Rng};
use std::sync::mpsc;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Neuron {
    // id: i32,
    v_threshold: f64,
    v_rest: f64,
    v_mem: f64, //la struct dovrà essere mutabile cosicchè ogni volta v_mem cambia in base al t
    v_reset: f64,
    connections_same_layer: Vec<f64>,
    connections_prec_layer: Vec<f64>
}

impl Neuron{

    pub fn new( v_threshold:f64, v_rest:f64, v_mem:f64, v_reset:f64, connections_same_layer: Vec<f64>,connections_prec_layer: Vec<f64>) -> Self{
        Neuron {
            // id,
            v_rest,
            v_threshold,
            v_mem, //valore t0
            v_reset,
            connections_same_layer,
            connections_prec_layer,
        }
    }
    //
    // fn computeOutput(&mut self, outputs:Vec<i32>, weights:Vec<f64>) -> i32{ //sarà chiamata dalla rete grande
    //     self.v_mem = self.v_rest + (self.v_mem - self.v_rest)*f64::exp(-1.0/0.1);
    //     for i in 0..outputs.len(){
    //         self.v_mem += outputs[i] as f64 * weights[i];
    //     }
    //
    //     if self.v_mem > self.v_threshold{
    //         self.v_mem = self.v_reset;
    //         return 1;
    //     }
    //     0
    // }
}

// impl fmt::Display for Neuron {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//
//         write!(f, "Neuron :  x: {}, y: {}", self.x, self.y)
//     }
// }



fn round_f64(n : f64) -> f64{
    (n * 100.0).round() / 100.0
}
