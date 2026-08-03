use nbody_simulation::{Body, Colour, Simulation, Vec2, Visualiser, simulation::Potential};

fn main() {
    let length: f64 = 20.0;
    let height: f64 = 20.0;
    let center = Vec2::new(length / 2.0, height / 2.0);   // (10.0, 10.0)

    let sun     = Body::new(1.0,     center + Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0),  Colour::YELLOW, 0.8, 1000);
    let mercury = Body::new(1.65e-7, center + Vec2::new(1.3, 0.0), Vec2::new(0.0, 5.51), Colour::RED,    0.06, 1000);
    let venus   = Body::new(2.45e-6, center + Vec2::new(1.7, 0.0), Vec2::new(0.0, 4.82), Colour::WHITE,  0.08, 1000);
    let earth   = Body::new(3.00e-6, center + Vec2::new(2.1, 0.0), Vec2::new(0.0, 4.34), Colour::BLUE,   0.08, 1000);
    let mars    = Body::new(3.21e-7, center + Vec2::new(2.6, 0.0), Vec2::new(0.0, 3.90), Colour::RED,    0.07, 1000);
    let jupiter = Body::new(9.55e-4, center + Vec2::new(3.6, 0.0), Vec2::new(0.0, 3.31), Colour::WHITE,  0.25, 1000);
    let saturn  = Body::new(2.86e-4, center + Vec2::new(4.6, 0.0), Vec2::new(0.0, 2.93), Colour::WHITE,  0.22, 1000);
    let uranus  = Body::new(4.37e-5, center + Vec2::new(5.4, 0.0), Vec2::new(0.0, 2.70), Colour::BLUE,   0.15, 1000);
    let neptune = Body::new(5.15e-5, center + Vec2::new(6.2, 0.0), Vec2::new(0.0, 2.52), Colour::BLUE,   0.15, 1000);

    let mut sim: Simulation = Simulation::new(height, length, 0.001, Potential::Gravity3d);
    sim.add_body(sun);
    sim.add_body(mercury);
    sim.add_body(venus);
    sim.add_body(earth);
    sim.add_body(mars);
    sim.add_body(jupiter);
    sim.add_body(saturn);
    sim.add_body(uranus);
    sim.add_body(neptune);

    let vis: Visualiser = Visualiser::new(sim, 600, 600).expect("Failed to open window");

    vis.run(10000000);
}