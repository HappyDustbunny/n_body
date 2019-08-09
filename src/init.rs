extern crate rand;

use std::io::Write;
use rand::prelude::*;

use super::hns;

const ERROR_MESSAGE: &'static str = r"
This program simulates a collision of two star clusters (CL1 and CL2)
As input it needs the number of stars in each cluster, their radii
 and the initial position and velocity of CL2.
CL1 starts at rest at origo.
Ex: n_body #_of_stars_CL1 #_of_stars_CL2 radiusCL1 radiusCL2 x y z vx vy vz
Ex: n_body 1500 100 3000.0 2000.0 6000.0 0.0 0.0 -1.0 0.0 0.0
";

pub fn sanitize(args: Vec<String>) -> std::vec::Vec<f32> {
    let mut sanitzed_args: Vec<f32> = Vec::new();

    if args.len() != 11 {
        writeln!(std::io::stderr(), "Wrong number of parameters or wrong type(s)").unwrap();
        println!("{:?}", args);

        writeln!(std::io::stderr(), "{}", ERROR_MESSAGE).unwrap();
        // writeln!(std::io::stderr(), "This program simulates a collision of two star clusters (CL1 and CL2)").unwrap();
        // writeln!(std::io::stderr(), "As input it needs the number of stars in each cluster, their radii").unwrap();
        // writeln!(std::io::stderr(), " and the initial position and velocity of CL2.").unwrap();
        // writeln!(std::io::stderr(), "CL1 starts at rest at origo.").unwrap();
        // writeln!(std::io::stderr(), "Ex: n_body #_of_stars_CL1 #_of_stars_CL2 radiusCL1 radiusCL2 x y z vx vy vz").unwrap();
        // writeln!(std::io::stderr(), "Ex: n_body 1500 100 3000.0 2000.0 6000.0 0.0 0.0 -1.0 0.0 0.0").unwrap();
        std::process::exit(1);
    }

    for num in 1..11 {
            sanitzed_args.push(args[num].parse().unwrap())
    }

    sanitzed_args
}

pub fn initialise_stars(number_of_stars: usize, radius_of_cluster: f32, color: [f32; 3]) -> Vec<hns::Star> {

    let mut stars: Vec<hns::Star> = vec![];

    for _ in 0..number_of_stars {
        let mut newstar = hns::Star::new();
        // newstar.mass=1.0;
        newstar.color = color;
        // Setting position of new star by choosing random polar coordinates inside sphere
        // of radius radius_of_cluster
        let phi: f32 = thread_rng().gen_range(0.0, 6.28);
        let theta: f32 = thread_rng().gen_range(-3.14, 3.14);
        newstar.pos=hns::Hector {
            x: radius_of_cluster*theta.sin()*phi.cos(),
            y: radius_of_cluster*theta.sin()*phi.sin(),
            z: radius_of_cluster*theta.cos(),
        };
        // Giving the cluster a slight rotation
        newstar.vel = hns::Hector {
            x: -newstar.pos.y/(0.2*radius_of_cluster),
            y: newstar.pos.x/(0.2*radius_of_cluster),
            z: 0.0
        };
        stars.push(newstar)
    }

    // let mut central_monster = hns::Star::new();
    // central_monster.mass = 200.0;
    // stars.push(central_monster);

    stars
}

pub fn set_center_and_vel(stars: &mut Vec<hns::Star>, cluster_center: hns::Hector, cluster_vel: hns::Hector){
    for star in stars {
        star.pos.add_change(&cluster_center);
        star.vel.add_change(&cluster_vel);
    };
}
