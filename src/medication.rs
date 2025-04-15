use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

pub const MEDICATION_TYPES: usize = 3;

// Now unused, but leaving in as reference (normally I'd delete it).
#[derive(Debug)]
pub(crate) enum MedicationType {
    A,
    B,
    C,
}

/// Randomly selects one of the medication types
impl Distribution<MedicationType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MedicationType {
        match rng.random_range(0..3) {
            0 => MedicationType::A,
            1 => MedicationType::B,
            _ => MedicationType::C,
        }
    }
}
