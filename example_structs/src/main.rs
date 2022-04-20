use rand::prelude::*;
use rand_distr::StandardNormal;
// Good way to bundle data -> structures

#[derive(Debug)]
struct MySim {
    dt: f64,
    std: f64,
    nsteps: usize,
    traj: Vec<f64>,
}
impl MySim {

    // Instantiate a new Simulation object
    fn new(nsteps: usize) -> Self {
        Self {
            nsteps: nsteps,
            std: 1.,
            dt: 0.01,
            traj: vec![0.],
        }
    }

    // integrate Brownian motion
    fn simulate(&mut self) {
        // integrate
        for __ in 0..self.nsteps {
            let noise: f64 = thread_rng().sample(StandardNormal);
            let next_frame = self.traj[self.traj.len() - 1] + (self.std * self.dt).sqrt() * noise;
            self.traj.push(next_frame);
        }
    }

}

fn main() {
    // instanciate struct
    let mut my_sim = MySim::new(100);
    // We can print it because we derive Debug
    println!("{:#?}", my_sim);

    // run simulation and print traj member of my_sim
    my_sim.simulate();
    println!("{:?}", my_sim.traj);

}
