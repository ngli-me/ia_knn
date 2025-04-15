use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

#[derive(Debug)]
pub(crate) enum MedicationType {
    A,
    B,
    C,
}

impl Distribution<MedicationType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MedicationType {
        match rng.random_range(0..3) {
            0 => MedicationType::A,
            1 => MedicationType::B,
            _ => MedicationType::C,
        }
    }
}
