use rand::prelude::*;
use rand_distr::StandardNormal;
use serde_derive::Deserialize;
use std::io::Read;

// Shared behaviour between structs -> traits

#[derive(Debug, Deserialize)]
struct Trajectory {
    x: Vec<f64>
}

// implement TRAIT for STRUCT
impl Default for Trajectory {

    fn default() -> Self {
        Self { x: vec![0.] }
    }

}

#[derive(Debug, Deserialize)]
struct MySim {
    dt: f64,
    std: f64,
    nsteps: usize,
    #[serde(default)]
    traj: Trajectory,
}
impl MySim {

    // load a struct from a .toml file with the appropriate fields
    fn from_toml(path: &str) -> MySim {
        // transform from string to path
        let path = std::path::Path::new(path);
        // open file
        let mut file = std::fs::File::open(path).unwrap();
        let mut config_toml = String::new();
        file.read_to_string(&mut config_toml).unwrap();
        let mysim: MySim = toml::from_str(&config_toml).unwrap();
        mysim
    }

    // integrate Brownian motion
    fn simulate(&mut self) {
        for __ in 0..self.nsteps {
            let noise: f64 = thread_rng().sample(StandardNormal);
            let next_frame = self.traj.x[self.traj.x.len() - 1] + (self.std * self.dt).sqrt() * noise;
            self.traj.x.push(next_frame);
        }
    }

}


fn main() {
    // instanciate struct
    let mut my_sim = MySim::from_toml("sim.toml");
    // We can print it because we derive Debug
    println!("{:#?}", my_sim);

    // run simulation and print traj member of my_sim
    my_sim.simulate();
    println!("{:?}", my_sim.traj.x);
}
