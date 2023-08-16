use rand::{thread_rng, Rng};
use std::fmt;

//#[derive(Debug)]
// pub struct Connection {
//     id_input: i32,
//     weight: f64
// }
//
// impl Connection {
//
//     pub fn new(id_input:i32, weight:f64) -> Self{
//         Connection{
//             id_input,
//             weight
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum ErrorComponent {
    NoErr,
    Threshold,
    VMem,
    Weights
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    None,
    Stuck0,
    Stuck1,
    BitFlip
}

#[derive(Debug, Clone, Copy)]
pub struct ConfErr {
    id_neuron: i32,
    t_start: i32,
    duration: i32, //valutare se aggiungere t_end cosi da avere sempre vincolo dentro boundaries (generi da t_start+1 a input.len())
    //counter_duration: i32,
    n_bit: i32,
    err_type: Type,
    err_comp: ErrorComponent,
    pub v_start: f64
}

impl ConfErr{

    pub fn new(id_neuron: i32, t_start:i32, duration:i32, /*counter_duration: i32,*/ n_bit:i32, err_type: Type, err_comp: ErrorComponent, v_start: f64) -> Self{
        ConfErr{
            id_neuron,
            t_start,
            duration,
            //counter_duration,
            n_bit,
            err_type,
            err_comp,
            v_start
        }
    }
}

#[derive(Debug, Clone)]
pub struct Neuron {
    id: i32,
    v_threshold: f64,
    v_rest: f64,
    v_mem: f64, //la struct dovrà essere mutabile cosicchè ogni volta v_mem cambia in base al t
    v_reset: f64,
    connections_same_layer: Vec<f64>,
    connections_prec_layer: Vec<f64>
}

impl Neuron{

    pub fn new( id : i32, v_threshold:f64, v_rest:f64, v_mem:f64, v_reset:f64, connections_same_layer: Vec<f64>,connections_prec_layer: Vec<f64>) -> Self{

        Neuron {
            id,
            v_threshold,
            v_rest,
            v_mem, //valore t0
            v_reset,
            connections_same_layer,
            connections_prec_layer,
        }
    }

    pub fn compute_output(&mut self, inputs_prec_layer: &Vec<i32>, inputs_same_layer: &Vec<i32>, errors_vec: &mut Vec<ConfErr>, time: i32) -> i32{ //sarà chiamata dalla rete grande
        for error in errors_vec{
            if error.id_neuron == self.id && error.t_start <= time && error.t_start+error.duration >= time {
                println!("Neurone: {}, time: {}, before error: {}, v_start: {}",self.id, time, self.v_threshold, error.v_start);
                self.create_error(error, time);
                //println!("prova di error: {}", error.prova);
                //println!("after error: {}",self.v_threshold);
            }
        }
        self.v_mem = self.v_rest + (self.v_mem - self.v_rest)*f64::exp(-1.0/0.1);
        //let _v_m = self.v_mem.clone();

        for i in 0..inputs_prec_layer.len(){
            self.v_mem += inputs_prec_layer[i] as f64 * self.connections_prec_layer[i];

        }

       for i in 0..inputs_same_layer.len(){
           self.v_mem += inputs_same_layer[i] as f64 * self.connections_same_layer[i];
        }


        // println!("id : {}, v_mem : {} -> {}", self.id, v_m, self.v_mem);
        if self.v_mem > self.v_threshold{
            self.v_mem = self.v_reset;
            return 1;
        }
        0
    }

    fn create_error(&mut self, error: &mut ConfErr, time: i32){
        let mut number: f64 = 0.0;
        let bit_position = error.n_bit; // Posizione del bit da modificare
        let mut ending = false;
        // Converte il numero in un intero e modifica il bit alla posizione desiderata
        match error.err_comp {
            ErrorComponent::Threshold => {
                if error.t_start==time { error.v_start = self.v_threshold; }
                if error.t_start+error.duration==time { self.v_threshold=error.v_start; ending=true; }
                number = self.v_threshold; },
            ErrorComponent::VMem => {
                if error.t_start==time { error.v_start = self.v_mem; }
                if error.t_start+error.duration==time { error.v_start = self.v_mem; ending=true; }
                number = self.v_mem; },
            _ => {println!("codione sempre prob")},
        }

        if ending==true{
            return;
        }

        let mut bits: u64 = number.to_bits();
        match error.err_type {
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
            },
            _ => {println!("greve")}
        }

        // Converte nuovamente gli "bits di floating point" in un f64 modificato
        number = f64::from_bits(bits);

        println!("Modified number: {}", number);
        match error.err_comp {
            ErrorComponent::Threshold => { self.v_threshold = number; },
            ErrorComponent::VMem => { self.v_mem = number; },
            _ => {println!("codione sempre prob")},
        }
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: &str = "world";
        //
        // owned_string.push_str(borrowed_string);
        // println!("{}", owned_string);


        let mut s1 = "[ ".to_owned();
        for i in &self.connections_same_layer{
            // s1 = s1 + &round_f64(i).to_string();
            s1 = s1 +  format!("{:.2}", i).as_str() ;

            s1 = s1 + ", ";
        }
        s1.pop();
        s1.pop();
        s1 = s1 + " ]";

        let mut s2 = "[ ".to_owned();
        for i in &self.connections_prec_layer{
            // s1 = s1 + &round_f64(i).to_string();
            s2 = s2 +  format!("{:.2}", i).as_str() ;

            s2 = s2 + ", ";
        }
        s2.pop();
        s2.pop();
        s2 = s2 + " ]";


        write!(f, "Neuron : id : {}, v_rest : {}, v_threshold : {}, v_mem  : {}, v_reset : {}, connections_same_layer : {}, connections_prec_layer : {}",
               self.id,
               round_f64(self.v_rest),
               round_f64(self.v_threshold),
               round_f64(self.v_mem),
               round_f64(self.v_reset),
                s1, s2)
    }
}





fn round_f64(n : f64) -> f64{
    (n * 100.0).round() / 100.0
}

