mod vector;
use vector::Vec2;

mod simulation;
use simulation::{Body, Simulation};

mod visualise;
use visualise::Visualiser;


fn main() {
    let length: f64 = 10.0;
    let height: f64 = 10.0; 

    let body1: Body = Body::new(1.0, Vec2::new(5.0, 5.0), Vec2::new(0.0, 0.0));
    let body2: Body = Body::new(0.0005, Vec2::new(7.0, 5.0), Vec2::new(0.0, 3.0));
    let body3: Body = Body::new(0.5, Vec2::new(1.0, 5.0), Vec2::new(0.0, -1.0));

    let mut sim: Simulation = Simulation::new(height, length, 0.001);
    sim.add_body(body1);
    sim.add_body(body2);
    sim.add_body(body3);

    let mut vis: Visualiser = Visualiser::new(sim, 600, 600).expect("Failed to open window");

    vis.run(10000);
}
