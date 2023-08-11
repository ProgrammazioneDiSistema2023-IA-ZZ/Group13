extern crate rand;
use std::fmt;
use rand::{thread_rng, Rng};
use std::sync::mpsc;

mod network;
mod neuron;
mod layer;

use network::Network;

fn main() {
    let network = Network::new( vec![3, 2, 2] );
    network.print_network();

}


//
// fn main() {
// let mut v = vec![vec![0; 3]; 4];
// v[0][0] = 1;
// let x = Neuron::new(1,v,2,2);
// let mut sender = Vec::new();
// let mut receiver = Vec::new();
// let mut threads = Vec::new();
// for i in 0..4 {
// let (s, r) = mpsc::channel::<i32>();
// sender.push(s);
// receiver.push(r);
// }
// for i in 0..4 {
//
// let tx = sender[i].clone();
// let rx = receiver[i].clone();
// threads[i] = thread::spawn(move || {
//
//
//
//
// tx.send(i as i32).unwrap();
// });
//
// }
//
//
//
//
//
// let (sender, rx) =   mpsc::channel::<i32>();
//
// let tx = sender.clone();
// let t1 = thread::spawn(move || {
//
// tx.send(1).unwrap();
// tx.send(3).unwrap();
//
// });
//
// let tx = sender.clone();
// let t2 = thread::spawn(move || {
// tx.send(2).unwrap();
// });
//
// let t3 = thread::spawn(move || {
// println!("Got: {}", rx.recv().unwrap() );
// println!("Got: {}", rx.recv().unwrap() );
// println!("Got: {}", rx.recv().unwrap() );
//
// });
//
// t1.join().unwrap();
// t2.join().unwrap();
// t3.join().unwrap();
// }
