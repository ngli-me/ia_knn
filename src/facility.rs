use crate::medication::MedicationType;
use std::fmt;
use std::fmt::Formatter;

pub struct Facility {
    id: usize,
    inventory: MedicationType,
    price: f64,
}

impl Facility {
    pub(crate) calc_distance(&self, x: usize, y: usize) {

    }

    pub(crate) generate() -> [] {

    }
}

impl fmt::Display for Facility {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Central Fill {:03} - ${:.2}, Medication {:?}",
            self.id, self.price, self.inventory
        )
    }
}
