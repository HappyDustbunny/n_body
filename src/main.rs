extern crate rand;
use rand::prelude::*;
use nannou::prelude::*;


extern crate n_body;
use n_body::hns;

fn main() {
    // static NUMBER_OF_STARS: usize = 32; // Number of stars
    // let timestep = 0.1; // Time in Mega year
    //
    // let stars: Vec<hns::Star> = initialise_stars(NUMBER_OF_STARS);
    // let mut sectors = make_sectors(stars, 3);
    // for _k in 0..2 {
    //     let mut sectors_as_stars = Vec::new();
    //     let mut stars = Vec::new();
    //     for sec in &sectors {
    //         sectors_as_stars.push(sec.as_star())
    //     }
    //     for mut sec in sectors {
    //         sec.acc_reset();
    //         sec.internal_acc();
    //         for sas in &sectors_as_stars {
    //             sec.external_acc(sas);
    //         }
    //         for star in sec.star_list {
    //             stars.push(star);
    //         }
    //     }
    //     println!("Loop {:?}", _k);
    //     for star in &mut stars {
    //         star.find_vel(timestep);
    //         star.find_pos(timestep);
    //         star.print_stats();
    //     }
    //     sectors = make_sectors(stars, 3);
    // }
    nannou::sketch(view);
}

fn view(app: &App, frame:Frame) -> Frame {
    app.main_window().set_inner_size_points(720.0, 720.0);

    let draw = app.draw();
    draw.background().color(BLACK);
    draw.ellipse().x_y(50.0, 50.0).radius(1.0).color(WHITE);

    static NUMBER_OF_STARS: usize = 50; // Number of stars
    let timestep = 5.5; // Time in Mega year

    let stars: Vec<hns::Star> = initialise_stars(NUMBER_OF_STARS);
    let mut sectors = make_sectors(stars, 3);
    for _k in 0..300 {
        let mut sectors_as_stars = Vec::new();
        let mut stars = Vec::new();
        for sec in &sectors {
            sectors_as_stars.push(sec.as_star())
        }
        for mut sec in sectors {
            sec.acc_reset();
            sec.internal_acc();
            for sas in &sectors_as_stars {
                sec.external_acc(sas);
            }
            for star in sec.star_list {
                stars.push(star);
            }
        }
        println!("Loop {:?}", _k);
        for star in &mut stars {
            star.find_vel(timestep);
            star.find_pos(timestep);
            // star.print_stats();
            draw.ellipse().x_y(star.pos.x, star.pos.y).radius(1.0);
        }
        sectors = make_sectors(stars, 3);
    }

    draw.to_frame(app, &frame).unwrap();

    frame
}

fn initialise_stars(number_of_stars: usize) -> Vec<hns::Star> {
    let radius_of_cluster: f32 = 150.0;
    let mut stars: Vec<hns::Star> = vec![];

    for _ in 0..number_of_stars {
        let mut newstar = hns::Star::new();
        newstar.mass=1.0;
        newstar.pos=hns::Hector {
            x: thread_rng().gen_range(-radius_of_cluster, radius_of_cluster),
            y: thread_rng().gen_range(-radius_of_cluster, radius_of_cluster),
            z: thread_rng().gen_range(-radius_of_cluster, radius_of_cluster)
        };
        newstar.vel = hns::Hector {
            x: -newstar.pos.y/(10.0*radius_of_cluster),
            y: newstar.pos.x/(10.0*radius_of_cluster),
            z: 0.0
        };
        stars.push(newstar)
    }
    stars
}

fn make_sectors(mut star_list: Vec<hns::Star>, recursions_left: u32) -> Vec<hns::Sector> {
    if recursions_left > 0 {
        if 2u32.pow(recursions_left) as usize > star_list.len() {
            panic!("The recursion depth {:?} is greater than the number of stars {:?}", recursions_left, star_list.len());
        }
        if recursions_left % 3 == 0 {
            star_list.sort_by(|a, b| a.pos.x.partial_cmp(&b.pos.x).unwrap());
        } else if recursions_left % 3 == 1 {
            star_list.sort_by(|a, b| a.pos.y.partial_cmp(&b.pos.y).unwrap());
        } else {
            star_list.sort_by(|a, b| a.pos.z.partial_cmp(&b.pos.z).unwrap());
        }
        let (sub_list_1, sub_list_2) = star_list.split_at(star_list.len()/2);
        let mut sub_list_1 = make_sectors(sub_list_1.to_vec(), recursions_left - 1);
        let sub_list_2 = make_sectors(sub_list_2.to_vec(), recursions_left - 1);
        for sector in sub_list_2 {
            sub_list_1.push(sector);
        }
        sub_list_1
    } else {
        let mut return_sector = hns::Sector::new();
        return_sector.add_multiple_stars(star_list);
        vec!(return_sector)
    }
}

// fn find_center(star_list: Vec<hns::Star>) -> (hns::Star, hns::Hector, hns::Hector) {
//     let mut center = hns::Star::new();
//     center.mass = 0.0;
//     let mut max_hector = star_list[0].pos
//     let mut min_hector = star_list[0].pos
//     for star in star_list {
//         center.pos.add_change(star.pos.multiply(star.mass));
//         center.mass += star.mass;
//
//         if star.pos.x > max_hector.x {
//             max_hector.x = star.pos.x
//         }
//         if star.pos.y > max_hector.y {
//             max_hector.y = star.pos.y
//         }
//         if star.pos.z > max_hector.z {
//             max_hector.z = star.pos.z
//         }
//         if star.pos.x < min_hector.x {
//             min_hector.x = star.pos.x
//         }
//         if star.pos.y < min_hector.y {
//             min_hector.y = star.pos.y
//         }
//         if star.pos.z < min_hector.z {
//             min_hector.z = star.pos.z
//         }
//     }
//     center.pos.x /= center.mass;
//     center.pos.y /= center.mass;
//     center.pos.z /= center.mass;
//     (center, min_hector, max_hector)
// }

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
