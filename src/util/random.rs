extern crate rand;

use self::rand::{thread_rng, Rng};

pub fn rand_num() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0., 1.)
}
