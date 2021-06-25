fn main() {
    println!("Hello, world! Hi");


    let f = SinFxnODE{};

    let n = f.get_state_length();

    let mut state = vec![0f64;n];
    let mut prime = vec![0f64;n];
    let mut ext = vec![0f64;f.get_extended_state_length()];

    let mut t = 0_f64;

    state[0] = f.get_value(t);
    let dt = 0.0001_f64;

    let tf = 0.5;

    let mut n_steps = ((tf-t)/dt) as usize;

    n_steps += 1;

    println!("n_steps: {}",n_steps);

    for _i in 0..n_steps {
        let extra_data = f.get_prime(t, &state, &mut prime, &mut ext);

        println!("t: {} pred: {} act: {} err: {}",t,state[0],extra_data.val,state[0]-extra_data.val);

        t += dt;
        for j in 0..n{
            state[j] = state[j] + dt * prime[j];
        }
    }
}

fn cos(x : f64)->f64{
    return x.cos();
}

fn sin(x : f64)->f64{
    return x.sin();
}

impl Propagatable<ExtraCustomData> for SinFxnODE{
    fn get_state_length(&self) -> usize{
        return 1;
    } 

    fn get_extended_state_length(&self) ->usize {
        return 1;
    }

    fn get_prime(&self,t : f64,_state: &[f64], prime : &mut[f64],ext : &mut[f64]) -> ExtraCustomData{
        prime[0] = cos(t);
        ext[0]= self.get_value(t);
        let loc_val = ext[0];
        let loc_dval = prime[0]; 
        let extra_data = ExtraCustomData {val:loc_val,dval:loc_dval};
        return extra_data;
    }
}

struct ExtraCustomData{
    val : f64,
    dval : f64
}

struct SinFxnODE{
    
}

impl BasicFxn for SinFxnODE {
    fn get_value(&self, x : f64) -> f64{
        return sin(x);
    }
}

trait BasicFxn{
    fn get_value(&self, x : f64) -> f64;
}

trait Propagatable<T> {

    fn get_state_length(&self)->usize;

    fn get_extended_state_length(&self)->usize;

    fn get_prime(&self,t : f64,state: &[f64], prime : &mut[f64],ext : &mut[f64]) -> T;
    
}