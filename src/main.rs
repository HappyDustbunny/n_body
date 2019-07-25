extern crate rand;
use rand::prelude::*;
use nannou::prelude::*;


extern crate n_body;
use n_body::hns;

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
}


fn model(app: &App) -> Model {
    app.main_window().set_inner_size_points(720.0, 720.0);

    let draw = app.draw();
    draw.background().color(BLACK);

    let number_of_stars: usize = 500; // Number of stars
    let timestep = 0.5; // Time in Mega year

    let stars: Vec<hns::Star> = initialise_stars(number_of_stars);
    let sectors = make_sectors(stars, 3);
    Model {
        sectors,
        timestep
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    let mut sectors_as_stars = Vec::new();
    let mut stars = Vec::new();
    for sec in &m.sectors {
        sectors_as_stars.push(sec.as_star())
    }
    for sec in &mut m.sectors {
        sec.acc_reset();
        sec.internal_acc();
        for sas in &sectors_as_stars {
            sec.external_acc(sas);
        }
        for star in &sec.star_list {
            stars.push(*star);
        }
    }
    for star in &mut stars {
        star.find_vel(m.timestep);
        star.find_pos(m.timestep);
        // star.print_stats();
    }
    m.sectors = make_sectors(stars, 3);
}

fn view(app: &App, m: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Comment this out to activate tracks.
    for sector in &m.sectors{
        for star in &sector.star_list {
            draw.ellipse().x_y(star.pos.x, star.pos.y).radius(1.0);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn initialise_stars(number_of_stars: usize) -> Vec<hns::Star> {
    let radius_of_cluster: f32 = 150.0;
    let mut stars: Vec<hns::Star> = vec![];

    for _ in 0..number_of_stars {
        let mut newstar = hns::Star::new();
        newstar.mass=1.0;
        let phi = thread_rng().gen_range(0.0, 6.28);
        let theta = thread_rng().gen_range(-3.14, 3.14);
        newstar.pos=hns::Hector {
            x: radius_of_cluster*theta.sin()*phi.cos(),
            y: radius_of_cluster*theta.sin()*phi.sin(),
            z: radius_of_cluster*theta.cos(),
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
