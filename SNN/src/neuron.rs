extern crate rand;
use std::fmt;
use rand::{thread_rng, Rng};
use std::sync::mpsc;
use std::ops::Add;

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

    pub fn compute_output(&mut self, inputs_prec_layer : &Vec<i32>, inputs_same_layer : &Vec<i32>) -> i32{ //sarà chiamata dalla rete grande
        self.v_mem = self.v_rest + (self.v_mem - self.v_rest)*f64::exp(-1.0/0.1);

        for i in 0..inputs_prec_layer.len(){
            self.v_mem += inputs_prec_layer[i] as f64 * self.connections_prec_layer[i];
        }

       for i in 0..inputs_same_layer.len(){
            self.v_mem += inputs_same_layer[i] as f64 * self.connections_same_layer[i];
        }

        if self.v_mem > self.v_threshold{
            self.v_mem = self.v_reset;
            return 1;
        }
        0
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: &str = "world";
        //
        // owned_string.push_str(borrowed_string);
        // println!("{}", owned_string);


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


        write!(f, "Neuron :   v_rest : {}, v_threshold : {}, v_mem  : {}, v_reset : {}, connections_same_layer : {}, connections_prec_layer : {}",
               self.v_rest, self.v_threshold, self.v_mem, self.v_reset, s1, s2)
    }
}





fn round_f64(n : f64) -> f64{
    (n * 100.0).round() / 100.0
}
