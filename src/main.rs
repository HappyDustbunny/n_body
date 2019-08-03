extern crate rand;
extern crate crossbeam;
// use rand::prelude::*;
use nannou::prelude::*;
// use std::io::Write;

// extern crate n_body;
// use n_body::hns;
// use lib::init;
pub mod hns;
pub mod init;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
    // nannou::sketch(view);
}

struct Model {
    sectors: Vec<hns::Sector>,
    timestep: f32,
    divider: f32,
}


fn model(app: &App) -> Model {
    // Collecting and sanitizing arguments:
    let arguments: Vec<String> = std::env::args().collect();
    let args = init::sanitize(arguments);

    // if args.len() != 11 {
    //     writeln!(std::io::stderr(), "This program simulates a collision of two star clusters (CL1 and CL2)").unwrap();
    //     writeln!(std::io::stderr(), "As input it needs the number of stars in each cluster, their radii").unwrap();
    //     writeln!(std::io::stderr(), " and the initial position and velocity of CL2.").unwrap();
    //     writeln!(std::io::stderr(), "CL1 starts at rest at origo.").unwrap();
    //     writeln!(std::io::stderr(), "Ex: n_body #_of_stars_CL1 #_of_stars_CL2 radiusCL1 radiusCL2 x y z vx vy vz").unwrap();
    //     writeln!(std::io::stderr(), "Ex: n_body 1500 100 3000.0 2000.0 6000.0 0.0 0.0 -1.0 0.0 0.0").unwrap();
    //     std::process::exit(1);
    // }
    // println!("{:?}", args);

    app.main_window().set_inner_size_points(720.0, 720.0);

    let draw = app.draw();
    draw.background().color(BLACK);

    // let scale = 100.0; //Increases the distances and the speed of the simulation
    // let number_of_stars: usize = 500; // Number of stars
    let timestep = 10.0; //0.1 * scale; // Time in Mega year
    let divider = 50.0; //0.5 * scale; //Zoom factor

    // Initializing cluster 1
    let number_of_stars = args[1] as usize; //.trim().parse::<usize>().unwrap() as usize;
    let radius_of_cluster = args[3]; //.trim().parse::<f32>().unwrap();

    let mut stars: Vec<hns::Star> = init::initialise_stars(number_of_stars, radius_of_cluster, [0.5, 0.5, 1.0]);
    // Offsetting center and give initial velocity to cluster 1
    let cluster_center = hns::Hector{x:0.0, y:0.0, z:0.0};
    let cluster_vel = hns::Hector{x:0.0, y:0.0, z:0.0};
    init::set_center_and_vel(&mut stars, cluster_center, cluster_vel);

    // Initializing cluster 2
    let number_of_stars = args[2] as usize; //.trim().parse::<usize>().unwrap() as usize;
    let radius_of_cluster = args[4]; //.trim().parse::<f32>().unwrap();
    // let x = args[5].trim().parse::<f32>().unwrap();
    // let y = args[6].trim().parse::<f32>().unwrap();
    // let z = args[7].trim().parse::<f32>().unwrap();
    // let vx = args[8].trim().parse::<f32>().unwrap();
    // let vy = args[9].trim().parse::<f32>().unwrap();
    // let vz = args[10].trim().parse::<f32>().unwrap();

    let mut stars2: Vec<hns::Star> = init::initialise_stars(number_of_stars, radius_of_cluster, [1.0, 0.5, 0.5]);
    // Offsetting center and give initial velocity to cluster 2
    let cluster_center = hns::Hector{x:args[5], y:args[6], z:args[7]};
    let cluster_vel = hns::Hector{x:args[8], y:args[9], z:args[10]};
    init::set_center_and_vel(&mut stars2, cluster_center, cluster_vel);

    for star in stars2 {
        stars.push(star);
    }

    let sectors = make_sectors(stars, 6);
    Model {
        sectors: sectors,
        timestep: timestep,
        divider: divider,
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    let mut sectors_as_stars = Vec::new();
    let mut stars = Vec::new();
    for sec in &m.sectors {
        sectors_as_stars.push(sec.as_star())
    }

    let threads = 8;
    let sectors_per_group = m.sectors.len() / threads;
    {
        let timestep = m.timestep;
        let groups: Vec<&mut [hns::Sector]> = m.sectors.chunks_mut(sectors_per_group).collect();
        crossbeam::scope(|spawner|{
            for group in groups.into_iter() {
                spawner.spawn(|_| {
                    for sec in group {
                        sec.acc_reset();
                        sec.internal_acc();
                        for sas in &sectors_as_stars {
                            sec.external_acc(sas);
                        }
                        for star in &mut sec.star_list {
                            star.find_vel(timestep);
                            star.find_pos(timestep);
                        }
                    }
                });
            }
        }).unwrap();
    }

    // for sec in &mut m.sectors {
    //     sec.acc_reset();
    //     sec.internal_acc();
    //     for sas in &sectors_as_stars {
    //         sec.external_acc(sas);
    //     }
    //     for star in &mut sec.star_list {
    //         star.find_vel(m.timestep);
    //         star.find_pos(m.timestep);
    //         stars.push(*star);
    //     }
    // }

    for sec in &mut m.sectors {
        for star in &sec.star_list {
            stars.push(*star);
        }
    }
    m.sectors = make_sectors(stars, 6);
}

fn view(app: &App, m: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Comment this out to activate tracks.
    for sector in &m.sectors{
        for star in &sector.star_list {
            draw.ellipse().x_y(star.pos.x / m.divider, star.pos.z / m.divider).radius(1.0).color(Rgb::new(star.color[0], star.color[1], star.color[2]));
        }
    }
    draw.to_frame(app, &frame).unwrap();
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
