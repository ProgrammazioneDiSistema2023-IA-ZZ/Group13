extern crate rand;
use crate::rand::Rng;
use std::vec;
use std::sync::mpsc;
use std::thread;
use crate::errors::ConfErr;
use crate::layer::Layer;
use crate::{get_array_input, get_input_f64};
use crate::neuron::Neuron;


pub struct Network {
    pub layers: Vec<Layer>,
    pub network_conf : Vec<i32>,
    pub n_layers : usize,
    pub n_neurons: i32
}


impl Network{

    pub fn new_empty(network_conf: Vec<i32>) -> Self{// [3 2 3] [0-2 3-4 5-7]
        let n_layers = network_conf.len();
        let mut layers = Vec::new();
        let mut start_id = 0;
        for layer in 0..n_layers{
            let range = (start_id, start_id+network_conf[layer]-1);
            start_id = start_id+network_conf[layer];
            layers.push(Layer::new_empty(range));
        }
        let n_neurons = network_conf.clone().iter().sum();
        Network{
            layers,
            network_conf,
            n_layers,
            n_neurons,
        }
    }

    pub fn add_random_neurons(&mut self,funzione: fn(&mut Neuron,&Vec<i32>,&Vec<i32>)->i32){ // [3 2 3]  [0 3 5]
        let mut rnd = rand::thread_rng();
        let mut id=0;
        for (index, layer) in self.layers.iter_mut().enumerate(){
            for _ in 0..self.network_conf[index]{
                layer.add_neuron(id,-52.0+rnd.gen_range(-1.0..=1.0),-65.0+rnd.gen_range(-1.0..=1.0),-65.0+rnd.gen_range(-1.0..=1.0),-60.0+rnd.gen_range(-1.0..=1.0),funzione);
                id+=1;
            }
        }
    }

    pub fn add_neurons(&mut self, funzione: fn(&mut Neuron,&Vec<i32>,&Vec<i32>)->i32){ // [3 2 3]  [0 3 5]
        let mut id=0;
        for (index, layer) in self.layers.iter_mut().enumerate(){
            for _ in 0..self.network_conf[index]{
                println!("write value for v_threshold, v_rest, v_mem, v_reset: for neuron {}", id);
                let values = get_array_input(4 as usize);
                layer.add_neuron(id,values[0],values[1],values[2],values[3],funzione);
                id+=1;
            }
        }
    }

    pub fn add_weights_from_input(&mut self,){
        for (index, layer) in self.layers.iter_mut().enumerate(){
            for n in 0..self.network_conf[index]{
                if index!=0{
                    println!("layer: {}, write {} prec_weights for neuron: {}",index,self.network_conf[index-1],n);
                    let prec_weights = get_array_input(self.network_conf[index-1] as usize);
                    layer.add_weights_prec_layer(n as usize,prec_weights);

                }else {
                    let input_weight =  get_input_f64(&format!("layer: {}, write 1 input weight for neuron: {}",index,n));
                    let mut prec_weights = vec![];
                    for i in 0..self.network_conf[index]{
                        if i==n {
                            prec_weights.push(input_weight);
                        }else {
                            prec_weights.push(0.0);
                        }
                    }
                    layer.add_weights_prec_layer(n as usize,prec_weights);
                }
                println!("layer: {}, write {} same_weights for neuron: {}",index,self.network_conf[index]-1,n);
                let same_weights = get_array_input((self.network_conf[index]-1) as usize);
                layer.add_weights_same_layer(n as usize,same_weights);
            }
        }
    }

    pub fn add_random_weights(&mut self){
        let mut id=0;
        for (index, layer) in self.layers.iter_mut().enumerate(){
            for n in 0..self.network_conf[index]{
                let weights = Layer::generate_weight(self.network_conf[index],if index as i32 ==0 {-1} else {self.network_conf[index-1]}, id);
                layer.add_weights_prec_layer(n as usize,weights.0);
                layer.add_weights_same_layer(n as usize,weights.1);
                id+=1;
            }
        }
    }

    // pub fn add_weights_same_layer(&mut self, layer: usize, id_in_layer: usize, connections_same_layer: Vec<f64>){
    //     self.layers[layer].add_weights_same_layer(id_in_layer,connections_same_layer);
    // }
    //
    // pub fn add_weights_prec_layer(&mut self, layer: usize, id_in_layer: usize, connections_prec_layer: Vec<f64>){
    //     self.layers[layer].add_weights_prec_layer(id_in_layer,connections_prec_layer);
    // }


    // pub fn new_random(network_conf: Vec<i32>) -> Self { //vettore in lunghezza indica numero layer ed il singolo valore indica quanti neuroni a lvl
    //     let mut layers = Vec::<Layer>::new();
    //     let n_layers = network_conf.len();
    //     let mut start_id = 0;
    //
    //     //chiama la funzione in layer che genera i layer con i neuroni
    //     for time in 0..n_layers {
    //         if time == 0 {
    //             layers.push(Layer::new_random_weight(start_id,network_conf[time], -1));
    //         } else {
    //             layers.push(Layer::new_random_weight(start_id,network_conf[time], network_conf[time - 1]));
    //         }
    //         start_id = start_id + network_conf[time];
    //     }
    //
    //     Network {
    //         layers,
    //         network_conf,
    //         n_layers,
    //     }
    // }

/***********************************************************************************************/

    pub fn create_thread(&mut self, inputs: Vec<Vec<i32>>,error : ConfErr) -> Vec<Vec<i32>> {

        let tot_time = inputs.len();
        let n_layers = self.n_layers;

        let mut sender = Vec::new();
        let mut receiver = Vec::new();
        for _ in 0..n_layers{
            let (s, r) = mpsc::channel::<Vec<i32>>();
            sender.push(s);
            receiver.push(r);
        }
        let (sender_output, receiver_output) = mpsc::channel::<Vec<i32>>();

        for i in 0..tot_time{
            sender[0].send(inputs[i].clone()).unwrap();
            println!("input {} : {:?}", i, inputs[i]);
        }

        /*************************************************************/

        let mut threads = Vec::new();
        for (layer, rec) in receiver.into_iter().enumerate() {
            let send;
            if layer == n_layers - 1 { //ultimo layer
                send = sender_output.clone();
            } else {
                send = sender[layer + 1].clone();
            }

            let n_neurons_in_layer = self.network_conf[layer];
            let mut layer_copy = self.layers[layer].clone();

            let handle = thread::spawn(move || {
                let mut input_same_layer = vec![0; n_neurons_in_layer as usize];

                for time in 0..tot_time {
                    let input_prec_layer = rec.recv().unwrap();

                    let output = layer_copy.compute_output(&input_prec_layer, &input_same_layer,  &error, time);

                    println!("thread {}, time : {}, input_same_layer : {:?}, input_prec_layer : {:?}, output : {:?}", layer, time, input_same_layer, input_prec_layer, output);
                    input_same_layer = output.clone();
                    send.send(output).unwrap();
                }
                layer_copy
            });

            threads.push(handle);
        }

        /*************************************************************/

        let mut outputs = Vec::new();
        for _ in 0..tot_time{
            outputs.push(receiver_output.recv().unwrap());
        }

        for t in threads {
            t.join().unwrap();
        }

        outputs
    }

/***********************************************************************************************/

    pub fn get_indexes(&self, id : i32)-> (usize, usize){

        for l in 0..self.n_layers{
            if self.layers[l].id_is_in_range(id){
                for (index, n) in self.layers[l].neurons.iter().enumerate(){
                    if n.id == id{
                        return (l, index);
                    }
                }
            }
        }
        (0,0)
    }


    pub fn print_network(&self){
        println!("Network :");
        for layer in &self.layers{
            println!("  Layer :");
            for neuron in &layer.neurons{
                println!("    {}", neuron);
            }
        }
    }

}








