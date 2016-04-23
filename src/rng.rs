use rand::thread_rng;
use rand::distributions::{ IndependentSample, Range };

// grabs a random sample from a range, max inclusive
pub fn inclusive_random(min: isize, max: isize) -> isize {
    let mut rng = thread_rng();
    let range = Range::new(min, max+1);
    
    range.ind_sample(&mut rng)
}

pub fn exclusive_random(max: isize) -> isize {
    let mut rng = thread_rng();
    let range = Range::new(0, max);
    
    range.ind_sample(&mut rng)
}