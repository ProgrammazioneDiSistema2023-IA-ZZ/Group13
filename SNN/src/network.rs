extern crate rand;
use std::{fmt, thread::JoinHandle, vec};
use rand::{thread_rng, Rng};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use crate::layer::{Layer};

pub struct Network{
    layers: Vec<Layer>,
    senders : Vec<Sender<Vec<i32>>>,
    receivers : Vec<Receiver<Vec<i32>>>,
}

pub fn thread_layer_function(){

}

impl Network{

    pub fn new(vec_neurons : Vec<i32>, length_input : i32) -> Self{ //vettore in lunghezza indica numero layer ed il singolo valore indica quanti neuroni a lvl
        let mut layers = Vec::<Layer>::new();
        let n_input = (&vec_neurons)[0];
        let n_output = (&vec_neurons)[vec_neurons.len()-1];
        let mut senders = Vec::new();
        let mut receivers = Vec::new();

        //chiama la funzione in layer che genera i layer con i neuroni
        for i in 0..vec_neurons.len() {
            //chiamata funzione
            if i == 0 {
                layers.push(Layer::new(vec_neurons[i], 1));
            } else {
                layers.push(Layer::new(vec_neurons[i], vec_neurons[i-1]));
            }
        }

        /*************************************************************/

        let mut sender = Vec::new();
        let mut receiver = Vec::new();
        for _ in 0..vec_neurons.len() {
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender.push(s);
            receiver.push(r);
        }

        let (sender_input, receiver_input) = mpsc::channel::<Vec<i32>>();
        let (sender_output, receiver_output) = mpsc::channel::<Vec<i32>>();

        /*************************************************************/

        // vettore di thread (1 per layer)
        let mut threads = vec![];
        for layer in 0..vec_neurons.len() {

            let mut send;
            let mut rec;
            if layer == 0 { //primo layer
                send = sender[layer].clone();
                rec =  &receiver_input;
            } else if layer == vec_neurons.len() - 1 { //ultimo layer
                send = sender_output.clone();
                rec = &receiver[layer-1];
            } else{
                send = sender[layer].clone();
                rec = &receiver[layer-1];
            }
            let mut current_layer = &layers[layer];
            let handle = thread::spawn(move || {
                //passi la funzione che deve gestire il calcolo per ogni layer e, di conseguenza, ogni neurone
                println!("thread {}", layer);


                let output = Vec::new();
                for j in 0..length_input{

                    let mut input_same_layer = if j==0 { vec![0 ; vec_neurons[layer] as usize ]}  else { output.clone() };
                    let input_prec_layer = rec.recv().unwrap();

                    let output = current_layer.compute_output(&input_prec_layer, &input_same_layer);

                    send.send(output);
                }
            });

            threads.push(handle);

        }

        for t in threads {
            t.join().unwrap();
        }

        Network{
            layers,
            senders,
            receivers,
        }


    }

    // pub fn compute_output(&mut self, layer : Layer, inputs_prec_layer : Vec<i32>, inputs_same_layer : Vec<i32>){
    //     layer.compute_output(inputs_prec_layer,inputs_same_layer);
    //
    // }

    // pub fn add_input(input : Vec<i32>){
    //     send_input( [0 1 0 1 1 1 -1] )
    // }

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











//
// for layer{
//
//     new layer(n_neurons, n_neurons_pre )
//
// }
//
// for layer_index 0..n_layer-1 {
//
//     (sender, receiver) = new channel
//
//     layers[layer_index].addSender( sender )
//     layers[layer_index+1].addReceiver( receiver )
//
// }
//
// for layer_index 0..n_layer-1 {
//     for neuron in neurons[layer_index]
//     (sender, receiver) = new channel
//
//     layers[layer_index].addSender( sender )
//     layers[layer_index+1].addReceiver( receiver )
//
// }
//
//
// new thread { move || fn(layer[i], sender, receiver)
//
//
// }
//
//




