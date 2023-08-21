extern crate rand;
use crate::rand::Rng;
use std::io;

mod network;
mod neuron;
mod layer;
mod errors;

use network::Network;
use errors::{Type, ErrorComponent, ConfErr};
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

                    network_test.add_neurons(lif);
                    network_test.add_random_weights();
                },
                false => { //by hand
                    println!("Genereting network with configured values and configured weights");

                    network_test.add_neurons(lif);
                    network_test.add_weights_from_input();
                }
            }
        }
    }
    network_test.print_network();

    let n_inputs = get_input("How long should simulation lasts (in instant of time)?");
    let mut inputs = Vec::new();
    let random_inputs: bool = get_yes_or_no("\nDo you want random inputs?");
    match random_inputs {
        true => {
            for _ in 0..n_inputs{
                inputs.push(gen_inputs(network_test.network_conf[0] as usize));
            }
        },
        false => {
            for i in 0..n_inputs{
                println!("Filling inputs instant {}:", i);
                inputs.push(get_array_input_i32(network_test.network_conf[0] as usize));
            }
        }
    }

    let errors_flag: bool = get_yes_or_no("\nDo you want to add some errors?");
    let mut num_inferences = 1;
    let mut error_type;
    match errors_flag {
        true => { 
            num_inferences = get_input("How many inferences do you want?");
            error_type = get_error_type();
        },   //yes errors
        false => {
            println!("No errors in the network");
            error_type = Type::None;
        } //Everything works fine
    }

    let err_comp = get_error_component();
    println!("\n*********************************************\n");
    let mut error = ConfErr::new_from_main(&network_test, Type::None,&vec![] ,  0);
    println!("Simulation without error: ");
    let outputs_no_err =  network_test.create_thread(inputs.clone(), error.clone());
    for j in 0..outputs_no_err.len(){
        println!("output {} : {:?}", j, outputs_no_err[j]);
    }
    println!("\n*********************************************\n");

    let mut count_err1 = 0;
    let mut count_err2 = 0;
    for i in 0..num_inferences{
        let mut error = ConfErr::new_from_main(&network_test, error_type, &err_comp ,n_inputs);
        println!("Simulation {}", i+1);
        let outputs =  network_test.create_thread(inputs.clone(), error.clone());
        for j in 0..outputs.len(){
            println!("output {} : {:?}", j, outputs[j]);
        }
        println!("\n*********************************************\n");

        count_err1 += compute_differences1(&outputs_no_err, &outputs);
        count_err2 += compute_differences2(&outputs_no_err, &outputs);
    }


    println!("resilience1: {:.2}", (num_inferences-count_err1)*100/num_inferences);
    println!("resilience2: {:.2}", (num_inferences*(outputs_no_err[0].len() * outputs_no_err.len())-count_err2)*100/(outputs_no_err[0].len() * outputs_no_err.len() * num_inferences) );







}











pub fn compute_differences1(right: &Vec<Vec<i32>>, output: &Vec<Vec<i32>>) -> usize{
    for i in 0..output.len(){
        for j in 0..output[i].len(){
            if right[i][j] != output[i][j]{
                return 1;
            }
        }
    }
    0
}

pub fn compute_differences2(right: &Vec<Vec<i32>>, output: &Vec<Vec<i32>>) -> usize{
    let mut count = 0;
    for i in 0..output.len(){
        for j in 0..output[i].len(){
            if right[i][j] != output[i][j]{
                count+=1;
            }
        }
    }
    count
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

fn get_binary_input(prompt: &str) -> i32 {
    loop {
        println!("{} (1/0)", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return 1,
            "0" => return 0,
            _ => println!("Invalid input. Please enter 1 or 0 only"),
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

fn get_error_component() -> Vec<ErrorComponent> {
    let mut err_cmp_vec = Vec::new();
    println!("Select error component for components list:");
    println!("1. Threshold");
    println!("2. VRest");
    println!("3. VMem");
    println!("4. VReset");
    println!("5. Weights");
    println!("6. Stop");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => err_cmp_vec.push(ErrorComponent::Threshold),
            "2" => err_cmp_vec.push(ErrorComponent::VRest),
            "3" => err_cmp_vec.push(ErrorComponent::VMem),
            "4" => err_cmp_vec.push(ErrorComponent::VReset),
            "5" => err_cmp_vec.push(ErrorComponent::Weights),
            "6" => return err_cmp_vec,
            _ => println!("Invalid input. Please select a valid option (1 ..= 6)."),
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

fn get_array_input_i32(size: usize) -> Vec<i32> {
    let mut numbers = Vec::new();

    for i in 0..size {
        let number = get_binary_input(&format!("Enter input for neuron {} (first layer):", i + 1));
        numbers.push(number as i32);
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