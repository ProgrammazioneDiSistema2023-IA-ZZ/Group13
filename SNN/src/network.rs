extern crate rand;
use std::fmt;
use rand::{thread_rng, Rng};
use std::sync::mpsc;

//use crate::layer::{Layer};

pub struct Network{
    n_neurons : i32,
    n_input : i32,
    n_output : i32,
    neurons: Vec<Vec<Neuron>>,
    //matrix: Vec<Vec<f64>> //la matrice é unica x tutti i neuroni, la colonna j rappresenta i pesi entranti nel neurone j
}

impl Network{

    pub fn new(vec_neurons:Vec<i32>) -> Self{ //vettore in lunghezza indica numero layer ed il singolo valore indica quanti neuroni a lvl
        let mut rng = thread_rng();
        let mut id = 0;
        let mut neurons = Vec::new();

        let mut vec_id_in_layers = Vec::new();
        for i in vec_neurons.clone(){
            let mut id_in_layer = Vec::new();
            for _ in 1..=i{
                id_in_layer.push(id);
                id+=1;
            }
            vec_id_in_layers.push(id_in_layer);
        }

        let n_neurons = id;
        let n_input = (&vec_neurons)[0];
        let n_output = (&vec_neurons)[vec_neurons.len()-1];

        /*************************************************************/

        let mut sender = Vec::new();
        let mut receiver = Vec::new();
        for id in 0..n_neurons{
            let (s, r) = mpsc::channel::<i32>();
            sender.push(s);
            receiver.push(r);
        }

        let mut sender_input = Vec::new();
        let mut receiver_input = Vec::new();
        for id in 0..n_input{
            let (s, r) = mpsc::channel::<i32>();
            sender_input.push(s);
            receiver_input.push(r);
        }

        let mut sender_output = Vec::new();
        let mut receiver_output = Vec::new();
        for id in 0..n_output{
            let (s, r) = mpsc::channel::<i32>();
            sender_output.push(s);
            receiver_output.push(r);
        }

        /*************************************************************/

        for layer  in 0..vec_id_in_layers.len(){
            neurons.push(Vec::new());

            for id in vec_id_in_layers[layer].clone() {

                let mut weights_prec = Vec::new();
                let mut weights_same = Vec::new();

                let mut receiver_prec = Vec::new();
                let mut receiver_same = Vec::new();
                let mut sender_post = Vec::new();

                if layer == 0{

                }else {
                    for prec_neuron  in vec_id_in_layers[layer-1].clone(){
                        weights_prec.push(Connection::new(prec_neuron, rng.gen::<f64>()));
                        receiver_prec.push(receiver[prec_neuron]);
                    }
                }

                for same_layer_neuron in vec_id_in_layers[layer].clone(){
                    if id != same_layer_neuron{
                        weights_same.push(Connection::new(same_layer_neuron, rng.gen::<f64>()));
                        receiver_same.push(receiver[same_layer_neuron] );
                    }
                }

                if layer < vec_id_in_layers.len()-1{
                    for post_neuron in vec_id_in_layers[layer + 1].clone(){
                        sender_post.push( sender[post_neuron].clone() );
                    }
                }



                neurons[layer].push(
                    Neuron::new(id,1.0,1.0,1.0,1.0,weights_same,weights_prec)
                );
            }
        }

        Network{
            n_neurons,
            n_input,
            n_output,
            neurons
        }
    }

    pub fn print_network(&self){
        println!("Network :");
        for layer in &self.neurons{
            println!("  Layer :");
            for neuron in layer{
                println!("    {:?}", neuron);
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




