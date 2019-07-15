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

    pub fn find_vel(&mut self, timestep: f32) {
        self.vel.add_change(&self.acc.multiply(timestep)); // Simple Euler integration: v = v + a*dt
    }

    pub fn find_pos(&mut self, timestep: f32) { // Simple Euler integration: s = s + v*dt
        self.pos.add_change(&self.vel.multiply(timestep));
    }

    pub fn acc_towards(&mut self, other_star: Star) {
        let distance = self.pos.multiply(-1.0).add(&other_star.pos);
        self.acc.add_change(&distance.multiply(other_star.mass/distance.length().powi(3)));
    }
}
