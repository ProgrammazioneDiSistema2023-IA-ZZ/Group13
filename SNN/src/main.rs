extern crate rand;

mod network;
mod neuron;
mod layer;

use network::Network;

fn main() {
    let mut network = Network::new( vec![3, 2, 2]);
    network.print_network();
    // network.print_a_neuron();

    println!("\n*********************************************\n");

    let mut inputs =Vec::new();
    inputs.push(vec![1;3]);
    inputs.push(vec![2;3]);
    let outputs = network.create_thread(inputs);

    for i in 0..outputs.len(){
        println!("output {} : {:?}", i, outputs[i]);
    }

    println!("\n*********************************************\n");

    network.print_network();

}

