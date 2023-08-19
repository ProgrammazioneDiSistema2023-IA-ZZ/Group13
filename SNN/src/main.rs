extern crate rand;
use crate::rand::Rng;
use std::io;
use std::ops::Index;

mod network;
mod neuron;
mod layer;
mod errors;

use network::Network;
use errors::Type;
use crate::layer::Layer;
use neuron::Neuron;

pub fn gen_inputs( n_input: usize)-> Vec<i32>{
    let mut rnd = rand::thread_rng();
    let mut input = Vec::new();
    for _ in 0..n_input{
        input.push((rnd.gen_range(0..10) as i32)%2);
    }
    input
}

fn main() {
    let mut rnd = rand::thread_rng();
    println!("Welcome to the Neural Network Configuration Menu!");
    let num_layers = get_input("\nEnter the number of layers: ");

    let mut network_conf = vec![0;num_layers];
    println!("\nNumber of neurons per layer: ");
    for i in 0..num_layers {
        let prompt = format!("-Layer {}: ", i);
        let num_neurons = get_input(&prompt);
        network_conf[i] = num_neurons as i32;
    }

    let mut network_test = Network::new_empty(network_conf.clone());

    let random_values: bool = get_yes_or_no("\nDo you want to generate random values for each neuron?");
    let random_weights: bool = get_yes_or_no("\nDo you want to generate random weights?");
    match random_values {
        true => { //random values
            match random_weights {
                true => { //random values
                    println!("Genereting network with random values and random weights");
                    // network_test = Network::new_random(network_conf);
                    network_test.add_random_neurons(lif);
                    network_test.add_random_weights();
                },
                false => { //by hand
                    println!("Genereting network with random values and configured weights");
                    network_test.add_random_neurons(lif);
                    network_test.add_weights_from_input();
                }
            }
        },
        false => { //by hand

            match random_weights {
                true => { //random values
                    println!("Genereting network with configured values and random weights");
                    println!("write value for v_threshold, v_rest, v_mem, v_reset: ");
                    let values = get_array_input(4 as usize);

                    network_test.add_neurons(values[0],values[1],values[2],values[3],lif);
                    network_test.add_random_weights();
                },
                false => { //by hand
                    println!("Genereting network with configured values and configured weights");
                    println!("write value for v_threshold, v_rest, v_mem, v_reset: ");
                    let values = get_array_input(4 as usize);

                    network_test.add_neurons(values[0],values[1],values[2],values[3],lif);
                    network_test.add_weights_from_input();
                }
            }
        }
    }
    network_test.print_network();

    let mut inputs = Vec::new();
    inputs.push(gen_inputs(network_test.network_conf[0] as usize));
    inputs.push(gen_inputs(network_test.network_conf[0] as usize));
    inputs.push(gen_inputs(network_test.network_conf[0] as usize));
    inputs.push(gen_inputs(network_test.network_conf[0] as usize));
    inputs.push(gen_inputs(network_test.network_conf[0] as usize));
    inputs.push(gen_inputs(network_test.network_conf[0] as usize));



    let errors_flag: bool = get_yes_or_no("\nDo you want to add some errors?");
    let num_errors;
    let error_type;
    match errors_flag {
        true => { 
            num_errors = get_input("How many errors do you want?");
            error_type = get_error_type();
        },   //yes errors
        false => {
            println!("No errors in the network");
            num_errors = 0;
            error_type = Type::None;
        } //Everything works fine
    }
    let outputs =  network_test.create_thread(inputs, error_type, num_errors as i32);
    for i in 0..outputs.len(){
        println!("output {} : {:?}", i, outputs[i]);
    }

    println!("\n*********************************************\n");

    network_test.print_network();

}

fn get_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn get_input_f64(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn get_yes_or_no(prompt: &str) -> bool {
    loop {
        println!("{} (y/n)", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please enter 'y' for yes or 'n' for no."),
        }
    }
}

fn get_error_type() -> Type {
    println!("Select the type of error:");
    println!("1. Stuck0");
    println!("2. Stuck1");
    println!("3. BitFlip");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Type::Stuck0,
            "2" => return Type::Stuck1,
            "3" => return Type::BitFlip,
            _ => println!("Invalid input. Please select a valid option (1, 2, or 3)."),
        }
    }
}

fn get_array_input(size: usize) -> Vec<f64> {
    let mut numbers = Vec::new();

    for i in 0..size {
        let number = get_input_f64(&format!("Enter number {}:", i + 1));
        numbers.push(number as f64);
    }

    numbers
}

pub fn lif(neuron :&mut Neuron, inputs_prec_layer: &Vec<i32>, inputs_same_layer: &Vec<i32>) -> i32{
    neuron.v_mem = neuron.v_rest + (neuron.v_mem - neuron.v_rest)*f64::exp(-neuron.delta_t/0.1);
    neuron.delta_t = 1.0;

    for i in 0..inputs_prec_layer.len(){
        neuron.v_mem += inputs_prec_layer[i] as f64 * neuron.connections_prec_layer[i];

    }

    for i in 0..inputs_same_layer.len(){
        neuron.v_mem += inputs_same_layer[i] as f64 * neuron.connections_same_layer[i];
    }

    if neuron.v_mem > neuron.v_threshold{
        neuron.v_mem = neuron.v_reset;
        return 1;
    }
    0
}