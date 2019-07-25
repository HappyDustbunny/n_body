
#[derive(Copy, Clone)]
pub struct Hector { // Mathematical vector is called Hector in order not to confuse it with a Rust
                // vector. We could probably use a crate, but this is more fun as an exercise.
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Hector {
    pub fn new() -> Hector {
        Hector {x:0.0, y:0.0, z:0.0}
    }
    // Adds another Hector to current Hector
    pub fn add_change(&mut self, other_hector: &Hector) {
        self.x += other_hector.x;
        self.y += other_hector.y;
        self.z += other_hector.z;
    }

    pub fn add(&self, other_hector: &Hector) -> Hector {
        Hector {
            x: self.x + other_hector.x,
            y: self.y + other_hector.y,
            z: self.z + other_hector.z
        }
    }

    pub fn multiply_change(&mut self, number: f32) {
        self.x *= number;
        self.y *= number;
        self.z *= number;
    }

    pub fn multiply(&self, number: f32) -> Hector {
        Hector {
            x: self.x * number,
            y: self.y * number,
            z: self.z * number
        }
    }

    pub fn divide_by_change(&mut self, number: f32) {
        self.x /= number;
        self.y /= number;
        self.z /= number;
    }

    pub fn divide_by(&self, number: f32) -> Hector {
        Hector {
            x: self.x / number,
            y: self.y / number,
            z: self.z / number
        }
    }

    pub fn cross(&self, other_hector: &Hector) -> Hector {
        Hector {
            x: (self.y * other_hector.z) - (self.z * other_hector.y),
            y: (self.z * other_hector.x) - (self.x * other_hector.z),
            z: (self.x * other_hector.y) - (self.y * other_hector.x)
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl PartialEq for Hector { //Makes it so you can see if A and B's xyz are the same with A == B
    fn eq(&self, other: &Hector) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

#[derive(Copy, Clone)]
pub struct Star {
    pub mass: f32,
    pub pos: Hector,
    pub vel: Hector,
    pub acc: Hector
}

impl Star {
    pub fn new() -> Star {
        Star {
            mass: 1.0,
            pos: Hector::new(),
            vel: Hector::new(),
            acc: Hector::new()
        }
    }

    pub fn find_vel(&mut self, timestep: f32) { // Simple Euler integration: v = v + a*dt
        self.vel.add_change(&self.acc.multiply(timestep));
    }

    pub fn find_pos(&mut self, timestep: f32) { // Simple Euler integration: s = s + v*dt
        self.pos.add_change(&self.vel.multiply(timestep));
    }

    pub fn acc_towards(&mut self, other_star: &Star) {
        if self.pos != other_star.pos {
            let distance = self.pos.multiply(-1.0).add(&other_star.pos);
            let distance = distance.multiply(other_star.mass/distance.length().powi(3));
            self.acc.add_change(&distance);
        }
    }

    pub fn print_stats(&self) {
        println!("M: {:?} P: {:?} {:?} {:?} V: {:?} {:?} {:?} A: {:?} {:?} {:?}",
        self.mass, self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z,
        self.acc.x, self.acc.y, self.acc.z);
    }
}

#[derive(Clone)]
pub struct Sector {
    pub total_mass: f32,
    pub center: Hector,
    pub star_list: Vec<Star>
}

impl Sector {
    pub fn new() -> Sector { //Makes a new, empty Sector.
        Sector {
            total_mass: 0.0,
            center: Hector::new(),
            star_list: Vec::new()
        }
    }

    pub fn add_star(&mut self, new_star: Star) { //Adds a single star to a Sector.
        self.total_mass += new_star.mass;
        self.center.add_change(&new_star.pos.multiply(new_star.mass).divide_by(self.total_mass));
        self.star_list.push(new_star);
    }

    pub fn add_multiple_stars(&mut self, new_stars: Vec<Star>) { //Adds multiple stars to a Sector
        let mut center_mod = Hector::new();
        for star in new_stars {
            self.total_mass += star.mass;
            center_mod.add_change(&star.pos.multiply(star.mass));
            self.star_list.push(star)
        }
        center_mod.divide_by_change(self.total_mass);
        self.center.add_change(&center_mod);
    }

    pub fn as_star(&self) -> Star { //Makes a Star with the Sector's properties.
        Star {
            mass: self.total_mass,
            pos: self.center,
            vel: Hector::new(),
            acc: Hector::new()
        }
    }

    pub fn acc_reset(&mut self) { //Sets the acceleration of all Stars in the Sector to 0.
        for n in 0..self.star_list.len() {
            self.star_list[n].acc = Hector::new();
        }
    }

    pub fn internal_acc(&mut self) { //Accelerates all Stars in the Sector towards each other.
        for n in 0..self.star_list.len() {
            let mut current_star = self.star_list[n];
            for m in 0..self.star_list.len() {
                current_star.acc_towards(&self.star_list[m]);
            }
            self.star_list[n] = current_star;
        }
    }

    pub fn external_acc(&mut self, target: &Star) { //Accelerates all Stars in the Sector
                                                    //towards the target. use as_star if you
                                                    //want to target another Sector.
        if self.center != target.pos {
            for n in 0..self.star_list.len() {
                self.star_list[n].acc_towards(target);
            }
        }
    }
}
