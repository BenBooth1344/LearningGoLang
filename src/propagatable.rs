
trait Propagatable<T> {

    fn get_state_length(&self)->usize;

    fn get_extended_state_length(&self)->usize;

    fn get_prime(&self,t : f64,state: &[f64], prime : &mut[f64],ext : &mut[f64]) -> T;
    
}