extern crate rand;
use crate::rand::Rng;
use std::vec;
use std::sync::mpsc;
use std::thread;
use crate::errors::{ConfErr,ErrorComponent,Type};
use crate::layer::Layer;


pub struct Network {
    layers: Vec<Layer>,
    network_conf : Vec<i32>,
    n_layers : usize,
}


impl Network{

    pub fn new(network_conf: Vec<i32>) -> Self { //vettore in lunghezza indica numero layer ed il singolo valore indica quanti neuroni a lvl
        let mut layers = Vec::<Layer>::new();
        let n_layers = network_conf.len();
        let mut start_id = 0;

        //chiama la funzione in layer che genera i layer con i neuroni
        for time in 0..n_layers {
            if time == 0 {
                layers.push(Layer::new(start_id,network_conf[time], -1));
            } else {
                layers.push(Layer::new(start_id,network_conf[time], network_conf[time - 1]));
            }
            start_id = start_id + network_conf[time];
        }

        Network {
            layers,
            network_conf,
            n_layers,
        }
    }

/***********************************************************************************************/

    pub fn create_thread(&mut self, inputs: Vec<Vec<i32>>, type_err: Type, n_err: i32) -> Vec<Vec<i32>> {

        let tot_time = inputs.len();
        let n_layers = self.n_layers;

        let mut sender = Vec::new();
        let mut receiver = Vec::new();
        for _ in 0..n_layers{
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender.push(s);
            receiver.push(r);
        }
        let (sender_output, receiver_output) = mpsc::channel::<Vec<i32>>();

        for i in 0..tot_time{
            sender[0].send(inputs[i].clone()).unwrap();
            println!("input {} : {:?}", i, inputs[i]);
        }

        let network_errors = ConfErr::network_create_errors(self.n_layers,n_err);
        println!("vec_errors: {:?}", network_errors);


        /*************************************************************/

        let mut threads = Vec::new();
        for (layer, rec) in receiver.into_iter().enumerate() {
            let send;
            if layer == n_layers - 1 { //ultimo layer
                send = sender_output.clone();
            } else {
                send = sender[layer + 1].clone();
            }

            let n_neurons_in_layer = self.network_conf[layer];
            let mut layer_copy = self.layers[layer].clone();
            let n_err_xlayer = network_errors[layer];
            // println!("copy {:?}", layer_copy);

            let handle = thread::spawn(move || {
                let mut input_same_layer = vec![0; n_neurons_in_layer as usize];

                let mut layer_errors = ConfErr::layer_create_error(layer_copy.range, type_err, n_err_xlayer, tot_time as i32);

                for time in 0..tot_time {
                    let input_prec_layer = rec.recv().unwrap();

                    let output = layer_copy.compute_output(&input_prec_layer, &input_same_layer, &mut layer_errors, time);

                    println!("thread {}, time : {}, input_same_layer : {:?}, input_prec_layer : {:?}, output : {:?}", layer, time, input_same_layer, input_prec_layer, output);
                    input_same_layer = output.clone();
                    send.send(output).unwrap();
                }
                layer_copy
            });

            threads.push(handle);
        }

        /*************************************************************/

        let mut outputs = Vec::new();
        for time in 0..tot_time{
            outputs.push(receiver_output.recv().unwrap());
        }

        for t in threads {
            t.join().unwrap();
        }

        outputs
    }

/***********************************************************************************************/

    pub fn print_network(&self){
        println!("Network :");
        for layer in &self.layers{
            println!("  Layer :");
            for neuron in &layer.neurons{
                println!("    {}", neuron);
            }
        }
    }

}








