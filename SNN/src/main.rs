extern crate rand;

mod network;
mod neuron;
mod layer;

use network::Network;
use crate::neuron::Type;


fn main() {
    let mut network = Network::new( vec![10, 7, 5, 6]);
    network.print_network();
    // network.print_a_neuron();

    println!("\n*********************************************\n");

    let mut inputs = Vec::new();
    inputs.push(vec![1;10]);
    inputs.push(vec![1;10]);
    inputs.push(vec![1;10]);
    let _outputs;// = network.create_thread(inputs.clone(), Type::None, 0);

   // for i in 0..outputs.len(){
        //println!("output {} : {:?}", i, outputs[i]);
    //}

    println!("\n*********************************************\n");

    network.print_network();

    //let mut error = ConfErr::new(2,1,0,2,0,16,Type::Stuck0,ErrorComponent::Threshold);
    _outputs = network.create_thread(inputs, Type::BitFlip, 3);
}
