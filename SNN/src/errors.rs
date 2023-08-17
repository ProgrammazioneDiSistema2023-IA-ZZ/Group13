use rand::Rng;

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

    pub fn create_errors(n_layers: usize, n_err: i32) -> Vec<i32>{
        let mut errors_vec = vec![0; n_layers];
        for _ in 0..n_err{
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..n_layers);
            errors_vec[x] += 1;
        }
        return errors_vec;
    }

}
