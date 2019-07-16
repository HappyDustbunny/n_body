extern crate rand;
use rand::prelude::*;

extern crate n_body;
use n_body::hns;

fn main() {
    static NUMBER_OF_STARS: usize = 10; // Number of stars
    let timestep = 0.1; // Time in Mega year

    let mut stars: Vec<hns::Star> = initialise_stars(NUMBER_OF_STARS);
    for _k in 0..2 {
        for n in 0..NUMBER_OF_STARS {
            let mut current_star = stars[n];
            for m in 0..NUMBER_OF_STARS {
                current_star.acc_towards(&stars[m]);
            }
            stars[n] = current_star;
        }
        for o in  0..NUMBER_OF_STARS {
            stars[o].find_vel(timestep);
            stars[o].find_pos(timestep);
            println!("{:?}", o);
            stars[o].print_stats();
        }
    }
}

fn initialise_stars(number_of_stars: usize) -> Vec<hns::Star> {
    let radius_of_cluster: f32 = 100.0;
    let mut stars: Vec<hns::Star> = vec![];

    for _item in 0..number_of_stars {
        let mut newstar = hns::Star::new();
        newstar.pos=hns::Hector {
            x: thread_rng().gen_range(0.0f32, radius_of_cluster),
            y: thread_rng().gen_range(0.0f32, radius_of_cluster),
            z: thread_rng().gen_range(0.0f32, radius_of_cluster)
        };
        newstar.vel = hns::Hector {
            x: -newstar.pos.y/radius_of_cluster,
            y: newstar.pos.x/radius_of_cluster,
            z: 0.0
        };
        stars.push(newstar)
    }
    println!("Yay");
    stars
}

// fn initialise_stars(number_of_stars: u32) -> Vec<Star> {
//     let radius_of_cluster: f32 = 100.0;
//     let mut stars: Vec<Star> = vec![];
//
//     for _item in 1..number_of_stars {
//         let mut newstar = Star::new();
//         newstar.pos=Hector {
//             x: thread_rng().gen_range(0.0f32, radius_of_cluster),
//             y: thread_rng().gen_range(0.0f32, radius_of_cluster),
//             z: thread_rng().gen_range(0.0f32, radius_of_cluster)
//         };
//         newstar.vel = Hector {
//             x: -newstar.pos.y/radius_of_cluster,
//             y: newstar.pos.x/radius_of_cluster,
//             z: 0.0
//         };
//         stars.push(newstar)
//     }
//     println!("Yay");
//     stars
// }
//
// pub struct Hector { // Mathematical vector is called Hector in order not to confuse it with a Rust
//                 // vector. We could probably use a crate, but this is more fun as an exercise.
//     pub x: f32,
//     pub y: f32,
//     pub z: f32
// }
//
// impl Hector {
//     pub fn new() -> Hector {
//         Hector {x:0.0, y:0.0, z:0.0}
//     }
//     // Adds another Hector to current Hector
//     pub fn add_change(&mut self, other_hector: &Hector) {
//         self.x += other_hector.x;
//         self.y += other_hector.y;
//         self.z += other_hector.z;
//     }
//
//     pub fn add(&self, other_hector: &Hector) -> Hector {
//         Hector {
//             x: self.x + other_hector.x,
//             y: self.y + other_hector.y,
//             z: self.z + other_hector.z
//         }
//     }
//
//     pub fn multiply_change(&mut self, number: f32) {
//         self.x *= number;
//         self.y *= number;
//         self.z *= number;
//     }
//
//     pub fn multiply(&self, number: f32) -> Hector {
//         Hector {
//             x: self.x * number,
//             y: self.y * number,
//             z: self.z * number
//         }
//     }
//
//     pub fn cross(&self, other_hector: &Hector) -> Hector {
//         Hector {
//             x: (self.y * other_hector.z) - (self.z * other_hector.y),
//             y: (self.z * other_hector.x) - (self.x * other_hector.z),
//             z: (self.x * other_hector.y) - (self.y * other_hector.x)
//         }
//     }
//
//     pub fn length(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
//     }
// }
//
// pub struct Star {
//     pub mass: f32,
//     pub pos: Hector,
//     pub vel: Hector,
//     pub acc: Hector
// }
//
// impl Star {
//     pub fn new() -> Star {
//         Star {
//             mass: 1.0,
//             pos: Hector::new(),
//             vel: Hector::new(),
//             acc: Hector::new()
//         }
//     }
//
//     pub fn find_vel(&mut self, timestep: f32) {
//         self.vel.add_change(&self.acc.multiply(timestep)); // Simple Euler integration: v = v + a*dt
//     }
//
//     pub fn find_pos(&mut self, timestep: f32) { // Simple Euler integration: s = s + v*dt
//         self.pos.add_change(&self.vel.multiply(timestep));
//     }
//
//     pub fn acc_towards(&mut self, other_star: Star) {
//         let distance = self.pos.multiply(-1.0).add(&other_star.pos);
//         self.acc.add_change(&distance.multiply(other_star.mass/distance.length().powi(3)));
//     }
// }
