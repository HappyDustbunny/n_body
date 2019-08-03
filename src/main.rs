extern crate rand;
extern crate crossbeam;
use rand::prelude::*;
use nannou::prelude::*;
use std::io::Write;

extern crate n_body;
use n_body::hns;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
    // nannou::sketch(view);
}

struct Model {
    cam_xy: WindowId,
    cam_xz: WindowId,
    cam_yz: WindowId,
    sectors: Vec<hns::Sector>,
    timestep: f32,
    divider: f32,
}


fn model(app: &App) -> Model {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 11 {
        writeln!(std::io::stderr(), "This program simulates a collision of two star clusters (CL1 and CL2)").unwrap();
        writeln!(std::io::stderr(), "As input it needs the number of stars in each cluster, their radii").unwrap();
        writeln!(std::io::stderr(), " and the initial position and velocity of CL2.").unwrap();
        writeln!(std::io::stderr(), "CL1 starts at rest at origo.").unwrap();
        writeln!(std::io::stderr(), "Ex: n_body #_of_stars_CL1 #_of_stars_CL2 radiusCL1 radiusCL2 x y z vx vy vz").unwrap();
        writeln!(std::io::stderr(), "Ex: n_body 1500 100 3000.0 2000.0 6000.0 0.0 0.0 -1.0 0.0 0.0").unwrap();
        std::process::exit(1);
    }
    println!("{:?}", args);

    // app.main_window().set_inner_size_points(720.0, 720.0);

    // let draw = app.draw();
    // draw.background().color(BLACK);

    let scale = 10.0; //Increases the distances and the speed of the simulation
    // let number_of_stars: usize = 500; // Number of stars
    let timestep = 1.0 * scale; // Time in Mega year
    let divider = 5.0 * scale; //Zoom factor
    // let radius_of_cluster =  30.0 *scale; //Radius of cluster
    let number_of_stars = args[1].trim().parse::<usize>().unwrap() as usize;
    let radius_of_cluster = args[3].trim().parse::<f32>().unwrap();

    let mut stars: Vec<hns::Star> = initialise_stars(number_of_stars, radius_of_cluster, [0.5, 0.5, 1.0]);
    let cluster_center = hns::Hector{x:0.0, y:0.0, z:0.0};
    let cluster_vel = hns::Hector{x:0.0, y:0.0, z:0.0};
    set_center_and_vel(&mut stars, cluster_center, cluster_vel);

    let number_of_stars = args[2].trim().parse::<usize>().unwrap() as usize;
    let radius_of_cluster = args[4].trim().parse::<f32>().unwrap();
    let x = args[5].trim().parse::<f32>().unwrap();
    let y = args[6].trim().parse::<f32>().unwrap();
    let z = args[7].trim().parse::<f32>().unwrap();
    let vx = args[8].trim().parse::<f32>().unwrap();
    let vy = args[9].trim().parse::<f32>().unwrap();
    let vz = args[10].trim().parse::<f32>().unwrap();

    let mut stars2: Vec<hns::Star> = initialise_stars(number_of_stars, radius_of_cluster, [1.0, 0.5, 0.5]);
    let cluster_center = hns::Hector{x:x, y:y, z:z};
    let cluster_vel = hns::Hector{x:vx, y:vy, z:vz};
    set_center_and_vel(&mut stars2, cluster_center, cluster_vel);


    let cam_xy = app
        .new_window()
        .with_dimensions(720, 720)
        .with_title("X Y")
        .build()
        .unwrap();
    let cam_xz = app
        .new_window()
        .with_dimensions(720, 720)
        .with_title("X Z")
        .build()
        .unwrap();
    let cam_yz = app
        .new_window()
        .with_dimensions(720, 720)
        .with_title("Y Z")
        .build()
        .unwrap();

    for star in stars2 {
        stars.push(star);
    }

    let sectors = make_sectors(stars, 6);
    Model {
        cam_xy: cam_xy,
        cam_xz: cam_xz,
        cam_yz: cam_yz,
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
    match frame.window_id() {
        id if id == m.cam_xy => {
            let draw_xy = app.draw_for_window(m.cam_xy).unwrap();
            draw_xy.background().color(BLACK); // Comment this out to activate tracks.
            for sector in &m.sectors{
                for star in &sector.star_list {
                    draw_xy.ellipse().x_y(star.pos.x / m.divider, star.pos.y / m.divider).radius(star.mass).color(Rgb::new(star.color[0], star.color[1], star.color[2]));
                }
            }
            draw_xy.to_frame(app, &frame).unwrap();
        },
        id if id == m.cam_xz => {
            let draw_xz = app.draw_for_window(m.cam_xz).unwrap();
            draw_xz.background().color(BLACK); // Comment this out to activate tracks.
            for sector in &m.sectors{
                for star in &sector.star_list {
                    draw_xz.ellipse().x_y(star.pos.x / m.divider, star.pos.z / m.divider).radius(star.mass).color(Rgb::new(star.color[0], star.color[1], star.color[2]));
                }
            }
            draw_xz.to_frame(app, &frame).unwrap();
        },
        id if id == m.cam_yz => {
            let draw_yz = app.draw_for_window(m.cam_yz).unwrap();
            draw_yz.background().color(BLACK); // Comment this out to activate tracks.
            for sector in &m.sectors{
                for star in &sector.star_list {
                    draw_yz.ellipse().x_y(star.pos.y / m.divider, star.pos.z / m.divider).radius(star.mass).color(Rgb::new(star.color[0], star.color[1], star.color[2]));
                }
            }
            draw_yz.to_frame(app, &frame).unwrap();
        },
        _ => (),
    }
}

fn initialise_stars(number_of_stars: usize, radius_of_cluster: f32, color: [f32; 3]) -> Vec<hns::Star> {
    // let radius_of_cluster: f32 = 3000.0;
    let mut stars: Vec<hns::Star> = vec![];

    for _ in 0..number_of_stars {
        let mut newstar = hns::Star::new();
        // newstar.mass=1.0;
        newstar.color = color;
        let phi = thread_rng().gen_range(0.0, 6.28);
        let theta = thread_rng().gen_range(-3.14, 3.14);
        newstar.pos=hns::Hector {
            x: radius_of_cluster*theta.sin()*phi.cos(),
            y: radius_of_cluster*theta.sin()*phi.sin(),
            z: radius_of_cluster*theta.cos(),
        };
        newstar.vel = hns::Hector {
            x: -newstar.pos.y/(0.21*radius_of_cluster),
            y: newstar.pos.x/(0.32*radius_of_cluster),
            z: 0.0
        };
        stars.push(newstar)
    }

    // let mut central_monster = hns::Star::new();
    // central_monster.mass = 200.0;
    // stars.push(central_monster);

    stars
}

fn set_center_and_vel(stars: &mut Vec<hns::Star>, cluster_center: hns::Hector, cluster_vel: hns::Hector){
    for star in stars {
        star.pos.add_change(&cluster_center);
        star.vel.add_change(&cluster_vel);
    };
}

fn make_sectors(mut star_list: Vec<hns::Star>, recursions_left: u32) -> Vec<hns::Sector> {
    if recursions_left > 0 {
        if 2u32.pow(recursions_left) as usize > star_list.len() {
            panic!("The recursion depth {:?} is greater than the number of stars {:?}", recursions_left, star_list.len());
        }
        star_list.sort_by(|a, b| match recursions_left % 3 {
            0 => a.pos.x.partial_cmp(&b.pos.x).unwrap(),
            1 => a.pos.y.partial_cmp(&b.pos.y).unwrap(),
            2 => a.pos.z.partial_cmp(&b.pos.z).unwrap(),
            _ => panic!("recursions_left % 3 somehow resulted in something other than 0, 1, or 2")
        });
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
