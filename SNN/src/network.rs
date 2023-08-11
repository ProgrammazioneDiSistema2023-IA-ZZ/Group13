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
    threads: Vec<JoinHandle<()>>,
    senders : Vec<Sender<Vec<i32>>>,
    receivers : Vec<Receiver<Vec<i32>>>,
}

impl Network{

    pub fn new(vec_neurons:Vec<i32>) -> Self{ //vettore in lunghezza indica numero layer ed il singolo valore indica quanti neuroni a lvl
        let mut layers = Vec::<Layer>::new();
        let n_input = (&vec_neurons)[0];
        let n_output = (&vec_neurons)[vec_neurons.len()-1];
        let mut senders = Vec::new();
        let mut receivers = Vec::new();

        //chiama la funzione in layer che genera i layer con i neuroni
        for i in 0..vec_neurons.len() {
            //chiamata funzione
            if i == 0 {
                layers.push(Layer::new(vec_neurons[i], -1));
            } else {
                layers.push(Layer::new(vec_neurons[i], vec_neurons[i-1]));
            }
        }

        /*************************************************************/

        let mut sender = Vec::new();
        let mut receiver = Vec::new();
        for id in 0..vec_neurons.len() {
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender.push(s);
            receiver.push(r);
        }

        let mut sender_input = Vec::new();
        let mut receiver_input = Vec::new();
        for id in 0..n_input{
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender_input.push(s);
            receiver_input.push(r);
        }

        let mut sender_output = Vec::new();
        let mut receiver_output = Vec::new();
        for id in 0..n_output{
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender_output.push(s);
            receiver_output.push(r);
        }

        /*************************************************************/

        // vettore di thread (1 per layer)
        let mut threads = vec![];
        for i in 0..vec_neurons.len() {
            let handle = thread::spawn(move || {
                //passi la funzione che deve gestire il calcolo per ogni layer e, di conseguenza, ogni neurone
                println!("OTOTOTOTOTOTOO");
            });
            threads.push(handle);
        }
        
        Network{
            layers,
            threads,
            senders,
            receivers,
        }
    }

    /*pub fn print_network(&self){
        println!("Network :");
        for layer in &self.neurons{
            println!("  Layer :");
            for neuron in layer{
                println!("    {:?}", neuron);
            }
        }
    }*/
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




