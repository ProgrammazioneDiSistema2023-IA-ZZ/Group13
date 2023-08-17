use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy)]
pub enum ErrorComponent {
    Threshold,
    VMem,
    Weights
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Stuck0,
    Stuck1,
    BitFlip
}

#[derive(Debug, Clone, Copy)]
pub struct ConfErr {
    pub id_neuron: i32,
    pub t_start: i32,
    pub duration: i32, //valutare se aggiungere t_end cosi da avere sempre vincolo dentro boundaries (generi da t_start+1 a input.len())
    //counter_duration: i32,
    pub n_bit: i32,
    pub err_type: Type,
    pub err_comp: ErrorComponent,
    pub w_pos: (i32, usize),
    pub original_parameter: f64
}

impl ConfErr{

    pub fn new(id_neuron: i32, t_start: i32, duration: i32, /*counter_duration: i32,*/ n_bit: i32, err_type: Type, err_comp: ErrorComponent, original_parameter: f64, w_pos: (i32, usize)) -> Self{
        ConfErr{
            id_neuron,
            t_start,
            duration,
            //counter_duration,
            n_bit,
            err_type,
            err_comp,
            original_parameter,
            w_pos
        }
    }

    pub fn is_overlapping(&self, t_start: i32, duration: i32) -> bool{
        (self.t_start < t_start+duration && self.t_start > t_start) || (self.t_start+self.duration > t_start && self.t_start < t_start )
    }

    pub fn network_create_errors(n_layers: usize, n_err: i32) -> Vec<i32>{
        let mut errors_vec = vec![0; n_layers];
        for _ in 0..n_err{
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..n_layers);
            errors_vec[x] += 1;
        }
        return errors_vec;
    }

    pub fn layer_create_error(range: (i32, i32), type_err: Type, n_errors: i32, tot_time: i32) -> Vec<ConfErr>{
        if n_errors <= 0{
            return vec![];
        }

        let mut rng = thread_rng();
        let mut errors_vec: Vec<ConfErr> = Vec::new();
        for _ in 0..n_errors{
            'loop_flag: loop{
                let id_n = rng.gen_range(range.0..=range.1);
                let t_start = rng.gen_range(0..tot_time);
                let duration;
                if let Type::BitFlip=type_err { duration=1 }
                else {duration = rng.gen_range(1..=3/*(tot_time-t_start)*/); }

                for e in &errors_vec{
                    if e.id_neuron == id_n && e.is_overlapping(t_start,duration){
                        continue 'loop_flag;
                    }
                }

                let bit = rng.gen_range(0..64);
                let flag = rng.gen_range(0..3);
                let cmpn;
                match flag {
                    0 => cmpn = ErrorComponent::Threshold,
                    1 => cmpn = ErrorComponent::VMem,
                    2 => cmpn = ErrorComponent::Weights,
                    _ => panic!("impossible")
                }

                let err = ConfErr::new(id_n,t_start,duration,bit,type_err,cmpn,0.0, (0,0));
                errors_vec.push(err);
                break 'loop_flag;
            }
        }
        errors_vec
    }

}
