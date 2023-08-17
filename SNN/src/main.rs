extern crate rand;
use crate::rand::Rng;

mod network;
mod neuron;
mod layer;

use network::Network;
use crate::neuron::Type;

pub fn gen_inputs( n_input: usize)-> Vec<i32>{
    let mut rnd = rand::thread_rng();
    let mut input = Vec::new();
    for _ in 0..n_input{
        input.push((rnd.gen_range(0..10) as i32)%2);
    }
    input
}

fn main() {
    let mut network = Network::new( vec![10, 7, 5, 6, 10, 4, 8, 5]);
    network.print_network();
    // network.print_a_neuron();

    println!("\n*********************************************\n");

    let mut inputs = Vec::new();
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));

    let _outputs;// = network.create_thread(inputs.clone(), Type::None, 0);

   // for i in 0..outputs.len(){
        //println!("output {} : {:?}", i, outputs[i]);
    //}

    println!("\n*********************************************\n");

    network.print_network();

    //let mut error = ConfErr::new(2,1,0,2,0,16,Type::Stuck0,ErrorComponent::Threshold);
    _outputs = network.create_thread(inputs, Type::Stuck1, 40);
}
