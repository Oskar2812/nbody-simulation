use nbody_simulation::{Body, Colour, Simulation, Vec2, Visualiser, simulation::Potential};

const NUM_THREADS: usize = 5;
fn main() {
     let length: f64 = 20.0;
    let height: f64 = 20.0;
    let center = Vec2::new(length / 2.0, height / 2.0);

    let central_mass: f64 = 100.0;
    let circular_speed: f64 = central_mass.sqrt();   // v = sqrt(G*M), independent of r for 1/r gravity

    let star = Body::new(central_mass, center, Vec2::new(0.0, -0.2), Colour::YELLOW, 0.6, 1000);

    let orbiter_a = Body::new(
        1.0,
        center + Vec2::new(3.0, 0.0),
        Vec2::new(0.0, circular_speed),
        Colour::BLUE,
        0.1,
        1000,
    );

    let orbiter_b = Body::new(
        1.0,
        center + Vec2::new(6.0, 0.0),
        Vec2::new(0.0, circular_speed),
        Colour::RED,
        0.1,
        1000,
    );

    let mut sim: Simulation = Simulation::new(height, length, 0.005, Potential::Gravity2d, NUM_THREADS);
    sim.add_body(star);
    sim.add_body(orbiter_a);
    sim.add_body(orbiter_b);

    let vis: Visualiser = Visualiser::new(sim, 600, 600).expect("Failed to open window");

    vis.run(200000);
}