extern crate rand;
use crate::rand::Rng;
use std::io;

mod network;
mod neuron;
mod layer;
mod errors;

use network::Network;
use errors::Type;

pub fn gen_inputs( n_input: usize)-> Vec<i32>{
    let mut rnd = rand::thread_rng();
    let mut input = Vec::new();
    for _ in 0..n_input{
        input.push((rnd.gen_range(0..10) as i32)%2);
    }
    input
}

fn main() {
    println!("Welcome to the Neural Network Configuration Menu!");
    let num_layers = get_input("\nEnter the number of layers: ");

    let mut layers = vec![0;num_layers];
    println!("\nNumber of neurons per layer: ");
    for i in 0..num_layers {
        let prompt = format!("-Layer {}: ", i);
        let num_neurons = get_input(&prompt);
        layers[i] = num_neurons as i32;
    }

    let mut network_test = Network {layers:vec![], network_conf: vec![], n_layers: 0};

    let random_generator: bool = get_yes_or_no("\nDo you want to generate random values for each neuron?");
    match random_generator {
        true => { network_test = Network::new_random( layers);},   //random values
        false => { println!("Call the function that, by hand, adds the values"); } //by hand
    }

    /*let random_values: bool = get_yes_or_no("\nDo you want to generate random values for each neuron?");
    match random_values {
        true => { println!("Call the function that generates random values"); },   //random values
        false => { println!("Call the function that, by hand, adds the values"); } //by hand
    }*/

    /*let random_weights: bool = get_yes_or_no("\nDo you want to generate random weights?");
    match random_weights {
        true => { println!("Call the function that generates random weights"); },   //random weights
        false => { println!("Call the function that, by hand, adds the weights"); } //by hand
    }*/

    let errors_flag: bool = get_yes_or_no("\nDo you want to add some errors?");
    match errors_flag {
        true => { 
            let num_errors = get_input("How many errors do you want?"); 
            let error_type = get_error_type(); 
        },   //yes errors
        false => { println!("No errors in the network"); } //Everything works fine
    }

    network_test.print_network();

    /*let mut network = Network::new_random( vec![10, 7, 5, 6, 10, 4, 8, 5]);
    network.print_network();

    println!("\n*********************************************\n");

    let mut inputs = Vec::new();
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));
    inputs.push(gen_inputs(10));

    // let _outputs = network.create_thread(inputs.clone(), Type::None, 0);

   // for i in 0..outputs.len(){
   //      println!("output {} : {:?}", i, outputs[i]);
   //  }

    // println!("\n*********************************************\n");

    // network.print_network();

    //let mut error = ConfErr::new(2,1,0,2,0,16,Type::Stuck0,ErrorComponent::Threshold);
    let outputs = network.create_thread(inputs, Type::Stuck1, 40);
    for i in 0..outputs.len(){
        println!("output {} : {:?}", i, outputs[i]);
    }*/
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
        let number = get_input(&format!("Enter number {}:", i + 1));
        numbers.push(number as f64);
    }

    numbers
}