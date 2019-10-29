use rand::{thread_rng, Rng};

// grabs a random sample from a range, max inclusive
pub fn inclusive_random(min: isize, max: isize) -> isize {
    thread_rng().gen_range(min, max + 1)
}

pub fn exclusive_random(max: isize) -> isize {
    thread_rng().gen_range(0, max)
}
