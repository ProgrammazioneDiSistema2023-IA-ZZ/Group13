extern crate rand;
use std::{fmt, thread::JoinHandle, vec};
use rand::{thread_rng, Rng};
use std::sync::{mpsc, Arc};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use crate::layer::{Layer};
use std::ops::Deref;

pub struct Network {
    layers: Vec<Layer>,
    vec_neurons : Vec<i32>
}


impl Network{

    pub fn new(vec_neurons : Vec<i32>) -> Self { //vettore in lunghezza indica numero layer ed il singolo valore indica quanti neuroni a lvl
        let mut layers = Vec::<Layer>::new();
        let n_layers = vec_neurons.len();
        let mut start_id = 0;

        //chiama la funzione in layer che genera i layer con i neuroni
        for i in 0..n_layers {
            if i == 0 {
                layers.push(Layer::new(start_id,vec_neurons[i], -1));
            } else {
                layers.push(Layer::new(start_id,vec_neurons[i], vec_neurons[i - 1]));
            }
            start_id = start_id + vec_neurons[i];
        }

        /*************************************************************/


        Network {
            layers,
            vec_neurons,
            // sender_input,
            // receiver_output,


        }
    }

/***********************************************************************************************/

    pub fn create_thread(&mut self, inputs : Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let length_input = inputs.len();
        let n_layers = self.vec_neurons.len();
        let mut sender = Vec::new();
        let mut receiver = Vec::new();
        for _ in 0..n_layers{
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender.push(s);
            receiver.push(r);
        }

        let (sender_output, receiver_output) = mpsc::channel::<Vec<i32>>();
        // let sender_input = sender[0].clone();
        /*************************************************************/

        for i in 0..length_input{
            sender[0].send(inputs[i].clone());
            println!("input {} : {:?}", i, inputs[i]);
        }

        /*************************************************************/

        let mut threads = Vec::new();
        for (layer, rec) in receiver.into_iter().enumerate() {
            let mut send;
            if layer == n_layers - 1 { //ultimo layer
                send = sender_output.clone();
            } else {
                send = sender[layer + 1].clone();
            }


            let n_neurons_in_layer = self.vec_neurons[layer];
            let mut layer_copy = self.layers[layer].clone();
            // println!("copy {:?}", layer_copy);

            let handle = thread::spawn(move || {
                let mut input_same_layer = vec![0; n_neurons_in_layer as usize];
                for j in 0..length_input {
                    let input_prec_layer = rec.recv().unwrap();

                    // let output = vec![input_prec_layer[0]+input_same_layer[0];input_prec_layer.len()];

                    let output= layer_copy.compute_output(&input_prec_layer,&input_same_layer);

                    println!("thread {}, j : {}, input_same_layer : {:?}, input_prec_layer : {:?}, o : {:?}", layer, j, input_same_layer, input_prec_layer, output);
                    input_same_layer = output.clone();
                    send.send(output);
                }
                layer_copy
            });

            threads.push(handle);
        }

        /*************************************************************/

        let mut outputs = Vec::new();
        outputs.push(receiver_output.recv().unwrap());
        outputs.push(receiver_output.recv().unwrap());

        let mut layers = Vec::new();
        for t in threads {
            layers.push(t.join().unwrap() );
        }
        self.layers = layers;
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

    pub fn print_a_neuron(&self){
        println!("n : {}", self.layers[0].clone_neuron());

    }

    // pub fn thread_join(&self){
    //     let mut x = 0;
    //     for t in self.threads.iter() {
    //         x = x + t.join().unwrap();
    //     }
    //
    //     println!("join : {}", x);
    // }




}








