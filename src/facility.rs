use std::collections::HashMap;
use crate::medication::MedicationType;
use rand::Rng;
use std::fmt;
use std::fmt::Formatter;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// Central Fill Facility Map
/// Contains a `HashMap` that contains the <Key, Value> pair of <ID, Facility>
pub struct FacilityMap {
    pub location: Vec<usize>,
    hm: HashMap<usize, Facility>,
    x_bound: usize,
    y_bound: usize,
}

impl FacilityMap {
    pub(crate) fn new(facility_count: usize, x_bound: usize, y_bound: usize) -> FacilityMap {
        let size = x_bound * y_bound;

        let mut location: Vec<usize> = vec![0; size];
        let mut hm: HashMap<usize, Facility> = HashMap::new();
        let mut rng: ThreadRng = rand::rng();

        // Scramble the index array for a 0-collision, seeded generation method
        let mut scramble: Vec<usize> = (0..size).collect();
        scramble.shuffle(&mut rng);

        for count in 0..facility_count {
            let id = rng.random_range(1..(size + 1));
            location[scramble[count]] = id;
            hm.insert(id, Facility::new(&mut rng));
        }

        FacilityMap {
            location,
            hm,
            x_bound,
            y_bound,
        }
    }

    pub(crate) fn calc_distance(&self, x: usize, y: usize) {

    }
}

pub struct Facility {
    inventory: MedicationType,
    price: f64,
}

impl Facility {
    fn new(mut rng: &mut ThreadRng) -> Facility {
        let inventory = rng.random::<MedicationType>();
        let price = rng.random::<f64>();
        Facility {
            inventory,
            price,
        }
    }
}

// Central Fill {:03} - , self.id
impl fmt::Display for Facility {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "${:.2}, Medication {:?}", self.price, self.inventory
        )
    }
}
