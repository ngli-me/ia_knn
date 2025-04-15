use crate::medication::MedicationType;
use priority_queue::PriorityQueue;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

/// Central Fill Facility Map
///
/// Contains a `HashMap` that contains the <Key, Value> pair of <Location, Facility>
pub struct FacilityMap {
    pub location: Vec<usize>, // Location is just for visualization, not needed for this calculation
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
            let x2 = scramble[count] / x_bound;
            let y2 = scramble[count] % y_bound;
            println!("{} {} {}", id, x2, y2);

            hm.insert(scramble[count], Facility::new(id, &mut rng));
        }

        FacilityMap {
            location,
            hm,
            x_bound,
            y_bound,
        }
    }

    // Calculate K Nearest Neighbors
    // Overall complexity of O((n * d * logn) + (3 * logn)) ~~ O(nlogn)
    pub(crate) fn knn(&self, k: usize, x1: usize, y1: usize) {
        let mut pq: PriorityQueue<usize, Reverse<usize>> = PriorityQueue::new();
        // Calculate all of the distances, O(n*d) -- but we're using a max heap
        // n = number of facilities
        // d = distance calculation complexity
        for idx in self.hm.keys() {
            let x2 = idx / self.x_bound;
            let y2 = idx % self.y_bound;
            //println!("{} {} {}", self.location[*idx], x2, y2);

            // Push the <item, priority> as <idx, Distance> onto the queue
            // Since we're using a max heap instead of looping, our complexity for insertion is O(logn)
            pq.push(*idx, Reverse(x1.abs_diff(x2) + y1.abs_diff(y2)));
        }
        // Overall complexity of O(logn * n), since indexing and distance are O(1)

        // Complexity of deleting/peeking is O(logn)
        pq.into_sorted_iter().take(k).for_each(|(idx, _)| {
            println!("{}", self.hm.get(&idx).unwrap());
        })
    }

    pub(crate) fn print(&self) {
        let mut axis_label = -(self.x_bound as i64) / 2;
        while axis_label < ((self.x_bound as i64) / 2) + 1 {
            print!("{:4}", axis_label);
            axis_label += 1;
        }
        println!();
        println!();

        let mut count: usize = 1;
        for val in &self.location {
            print!("{:4}", val);
            if count % self.x_bound == 0 {
                println!();
            }
            count += 1;
        }
    }
}

pub struct Facility {
    id: usize,
    inventory: MedicationType,
    price: f64,
}

impl Facility {
    fn new(id: usize, rng: &mut ThreadRng) -> Facility {
        let inventory = rng.random::<MedicationType>();
        let price = rng.random::<f64>();
        Facility {
            id,
            inventory,
            price,
        }
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
