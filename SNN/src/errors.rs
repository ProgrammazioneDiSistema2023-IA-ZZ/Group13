use rand::{Rng, thread_rng};
use crate::network::Network;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorComponent {
    None,
    Threshold,
    VRest,
    VMem,
    VReset,
    Weights,
    Multiplier,
    Adder,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    None,
    Stuck0,
    Stuck1,
    BitFlip
}

#[derive(Debug, Clone, Copy)]
pub struct ConfErr {
    pub id_neuron: i32,
    pub t_start: i32,
    pub n_bit: i32,
    pub err_type: Type,
    pub err_comp: ErrorComponent,
    pub w_pos: (i32, usize),
}

impl ConfErr{

    pub fn new(id_neuron: i32, t_start: i32, n_bit: i32, err_type: Type, err_comp: ErrorComponent, w_pos: (i32, usize)) -> Self{
        ConfErr{
            id_neuron,
            t_start,
            n_bit,
            err_type,
            err_comp,
            w_pos
        }
    }

   pub fn no_errors() -> Self{
        ConfErr{
            id_neuron : -1,
            t_start : -1,
            n_bit : -1,
            err_type : Type::None,
            err_comp : ErrorComponent::None,
            w_pos : (0,0)
        }
    }

    pub fn new_from_main(network: &Network, err_type: Type, err_comp: &Vec<ErrorComponent>, time: usize) -> Self{
        if err_type == Type::None{
            return ConfErr::no_errors();
        }

        let mut rng = thread_rng();
        let mut t_start = 0;
        let err_c = err_comp[rng.gen_range(0..err_comp.len())];

        if err_type == Type::BitFlip {
            t_start = rng.gen_range(0..time-1) as i32;
        }
        let id_neuron = rng.gen_range(0..network.n_neurons);

        let index;
        let vec = rng.gen_range(0..2);

        if err_c == ErrorComponent::Weights{
            let (layer, index_layer) = network.get_indexes(id_neuron);
            if vec==0 {//prec
                let len = network.layers[layer].neurons[index_layer].connections_prec_layer.len();
                index = rng.gen_range(0..len) as usize;
            }else {//same
                let len = network.layers[layer].neurons[index_layer].connections_same_layer.len();
                if len==0 { return ConfErr::no_errors(); }
                index = rng.gen_range(0..len) as usize;
            }
        }else{index = 0}

        ConfErr{
            id_neuron,
            t_start,
            n_bit: rng.gen_range(0..64),
            err_type,
            err_comp: err_c,
            w_pos: (vec, index),
        }
    }

    pub fn change_bit(&self, number: f64)-> f64{
        let bit_position = self.n_bit;
        let mut bits: u64 = number.to_bits();

        match self.err_type {
            Type::Stuck0 => {
                let mask = !(1 << bit_position);
                bits &= mask;// stuck ad 0
            },
            Type::Stuck1 => {
                let mask = 1 << bit_position;
                bits &= mask;// stuck ad 1
            },
            Type::BitFlip => {
                bits ^= 1 << bit_position; // Esegue un XOR per invertire il bit
            }
            Type::None => {
                panic!("impossible, NoError here!")
            }
        }
        f64::from_bits(bits)
    }

}

impl fmt::Display for ConfErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: Type: {:?}, Component: {:?}, IdNeuron: {}, Modified_bit: {}", self.err_type, self.err_comp, self.id_neuron, self.n_bit )
    }
}

