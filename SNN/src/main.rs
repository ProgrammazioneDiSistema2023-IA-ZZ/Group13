extern crate rand;

mod network;
mod neuron;
mod layer;

use network::Network;

fn main() {
    let mut network = Network::new( vec![10, 7, 5, 6]);
    network.print_network();
    // network.print_a_neuron();

    println!("\n*********************************************\n");

    let mut inputs =Vec::new();
    inputs.push(vec![1;10]);
    inputs.push(vec![1;10]);
    let outputs = network.create_thread(inputs);

    for i in 0..outputs.len(){
        println!("output {} : {:?}", i, outputs[i]);
    }

    println!("\n*********************************************\n");

    network.print_network();

}

