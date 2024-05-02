use rand::{rngs::StdRng, Rng, SeedableRng};


pub trait RandomNumberGenerator {
    fn next_random(&mut self)->u8;
} 

struct DefaultRandomNumberGenerator{
    RNG: StdRng
}

impl RandomNumberGenerator for DefaultRandomNumberGenerator {
    fn next_random(&mut self)->u8 {
        self.RNG.gen_range(0..37)
    }
}

impl Default for DefaultRandomNumberGenerator {
    fn default() -> DefaultRandomNumberGenerator {
        DefaultRandomNumberGenerator{
            RNG: StdRng::from_entropy()
        }
    }
}
