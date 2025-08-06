use rand::*;
use rand::distr::uniform::{SampleUniform, SampleRange};

#[allow(dead_code)]
pub fn int
    <T, R>
    (range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T> {

    let mut rng = rng();
    rng.random_range(range)
}
