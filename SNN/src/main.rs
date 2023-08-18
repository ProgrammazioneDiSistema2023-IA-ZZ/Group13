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

    let mut network_test = Network {layers:vec![], network_conf: vec![], n_layers: 0};
    let random_values: bool = get_yes_or_no("\nDo you want to generate random values for each neuron?");
    let random_weights: bool = get_yes_or_no("\nDo you want to generate random weights?");
    match random_values {
        true => { //random values
            match random_weights {
                true => { //random values
                    println!("Genereting network with random values and random weights");
                    network_test = Network::new_random(network_conf);
                },
                false => { //by hand
                    println!("Genereting network with random values and configured weights");
                    network_test = Network::new_empty(network_conf.clone());
                    let mut id=0;
                    for (index, layer) in network_test.layers.iter_mut().enumerate(){
                        for _ in 0..network_conf[index]{
                            layer.add_neuron(id,-52.0+rnd.gen_range(-1.0..=1.0),-65.0+rnd.gen_range(-1.0..=1.0),-65.0+rnd.gen_range(-1.0..=1.0),-60.0+rnd.gen_range(-1.0..=1.0));
                            id+=1;
                        }
                    }

                    for (index, layer) in network_test.layers.iter_mut().enumerate(){
                        for n in 0..network_conf[index]{
                            if index!=0{
                                println!("layer: {}, write {} prec_weights for neuron: {}",index,network_conf[index-1],n);
                                let prec_weights = get_array_input(network_conf[index-1] as usize);
                                layer.add_weights_prec_layer(n as usize,prec_weights);

                            }else {
                                let input_weight =  get_input_f64(&format!("layer: {}, write 1 input weight for neuron: {}",index,n));
                                let mut prec_weights = vec![];
                                for i in 0..network_conf[index]{
                                   if i==n {
                                       prec_weights.push(input_weight);
                                   }else {
                                       prec_weights.push(0.0);
                                   }
                                }
                                layer.add_weights_prec_layer(n as usize,prec_weights);
                            }
                            println!("layer: {}, write {} same_weights for neuron: {}",index,network_conf[index]-1,n);
                            let same_weights = get_array_input((network_conf[index]-1) as usize);
                            layer.add_weights_same_layer(n as usize,same_weights);
                        }
                    }
                }
            }
        },
        false => { //by hand
            match random_weights {
                true => { //random values
                    println!("Genereting network with configured values and random weights");
                    println!("write value for the 4 values of v:");
                    let values = get_array_input(4 as usize);

                    network_test = Network::new_empty(network_conf.clone());
                    let mut id=0;
                    for (index, layer) in network_test.layers.iter_mut().enumerate(){
                        for _ in 0..network_conf[index]{
                            layer.add_neuron(id,values[0],values[1],values[2],values[3]);
                            id+=1;
                        }
                    }

                    id=0;
                    for (index, layer) in network_test.layers.iter_mut().enumerate(){
                        for n in 0..network_conf[index]{
                            let weights = Layer::generate_weight(network_conf[index],if index as i32 ==0 {-1} else {network_conf[index-1]}, id);
                            layer.add_weights_prec_layer(n as usize,weights.0);
                            layer.add_weights_same_layer(n as usize,weights.1);
                            id+=1;
                        }
                    }
                },
                false => { //by hand
                    println!("Genereting network with configured values and configured weights");
                    println!("write value for the 4 values of v:");
                    let values = get_array_input(4 as usize);

                    network_test = Network::new_empty(network_conf.clone());
                    let mut id=0;
                    for (index, layer) in network_test.layers.iter_mut().enumerate(){
                        for _ in 0..network_conf[index]{
                            layer.add_neuron(id,values[0],values[1],values[2],values[3]);
                            id+=1;
                        }
                    }

                    for (index, layer) in network_test.layers.iter_mut().enumerate() {
                        for n in 0..network_conf[index] {
                            if index != 0 {
                                println!("layer: {}, write {} prec_weights for neuron: {}", index, network_conf[index - 1], n);
                                let prec_weights = get_array_input(network_conf[index - 1] as usize);
                                layer.add_weights_prec_layer(n as usize, prec_weights);

                            } else {
                                let input_weight = get_input_f64(&format!("layer: {}, write 1 input weight for neuron: {}", index, n));
                                let mut prec_weights = vec![];
                                for i in 0..network_conf[index] {
                                    if i == n {
                                        prec_weights.push(input_weight);
                                    } else {
                                        prec_weights.push(0.0);
                                    }
                                }
                                layer.add_weights_prec_layer(n as usize,prec_weights);
                            }
                            println!("layer: {}, write {} same_weights for neuron: {}", index, network_conf[index] - 1, n);
                            let same_weights = get_array_input((network_conf[index] - 1) as usize);
                            layer.add_weights_same_layer(n as usize, same_weights);
                        }
                    }
                }
            }
        }
    }

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