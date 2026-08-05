use nbody_simulation::{Body, Colour, Simulation, Vec2, Visualiser, simulation::Potential};

use std::f64::consts::PI;

const NUM_BODIES: usize = 20000;   // crank this up to stress-test compute_forces
const NUM_THREADS: usize = 5;

fn main() {
    let length: f64 = 40.0;
    let height: f64 = 40.0;
    let center = Vec2::new(length / 2.0, height / 2.0);

    let mut sim: Simulation = Simulation::new(height, length, 0.002, Potential::Gravity2d, NUM_THREADS);

    // simple deterministic pseudo-random generator so runs are repeatable
    let mut seed: u64 = 12345;
    let mut next_rand = move || -> f64 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as f64) / (u32::MAX as f64)
    };

    let mass = 1.0;
    let cluster_radius = 15.0;
    let max_initial_speed = 0.5;   // small random velocities, no orbital structure imposed

    for _ in 0..NUM_BODIES {
        // random position within a disk, using sqrt for uniform area coverage
        let angle = 2.0 * PI * next_rand();
        let radius = cluster_radius * next_rand().sqrt();

        let pos = center + Vec2::new(radius * angle.cos(), radius * angle.sin());

        let vel_angle = 2.0 * PI * next_rand();
        let speed = max_initial_speed * next_rand();
        let vel = Vec2::new(speed * vel_angle.cos(), speed * vel_angle.sin());

        let body = Body::new(mass, pos, vel, Colour::CYAN, 0.05, 0);
        sim.add_body(body);
    }

    sim.zero_total_momentum();

    let vis: Visualiser = Visualiser::new(sim, 800, 800).expect("Failed to open window");

    vis.run(200000);
}