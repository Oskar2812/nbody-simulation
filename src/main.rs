mod vector;
use vector::Vec2;

mod simulation;
use simulation::{Body, Simulation};

mod visualise;
use visualise::Visualiser;


fn main() {
    let length: f64 = 10.0;
    let height: f64 = 10.0; 

    let body: Body = Body::new(1.0, Vec2::new(5.0, 5.0), Vec2::new(0.001, 0.0));

    let mut sim: Simulation = Simulation::new(height, length, 1.0);
    sim.add_body(body);

    let mut vis: Visualiser = Visualiser::new(sim, 600, 600).expect("Failed to open window");

    vis.run(10000);

    println!("Halfway there");

    vis.run(10000);
}
