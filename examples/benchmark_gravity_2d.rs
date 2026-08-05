use nbody_simulation::{Simulation, Body, Vec2, Colour, simulation::Potential};
use std::time::Instant;
use std::f64::consts::PI;

fn build_random_cluster(num_bodies: usize) -> Simulation {
    let length: f64 = 40.0;
    let height: f64 = 40.0;
    let center = Vec2::new(length / 2.0, height / 2.0);

    let mut sim = Simulation::new(height, length, 0.002, Potential::Gravity2d);

    let mut seed: u64 = 12345;
    let mut next_rand = move || -> f64 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as f64) / (u32::MAX as f64)
    };

    for _ in 0..num_bodies {
        let angle = 2.0 * PI * next_rand();
        let radius = 15.0 * next_rand().sqrt();
        let pos = center + Vec2::new(radius * angle.cos(), radius * angle.sin());

        let vel_angle = 2.0 * PI * next_rand();
        let speed = 0.5 * next_rand();
        let vel = Vec2::new(speed * vel_angle.cos(), speed * vel_angle.sin());

        sim.add_body(Body::new(1.0, pos, vel, Colour::WHITE, 0.03, 0));
    }

    sim.zero_total_momentum();
    sim
}

fn benchmark(num_bodies: usize, num_steps: usize) {
    let mut sim = build_random_cluster(num_bodies);

    // warm-up step, excluded from timing (avoids counting any one-time setup cost)
    sim.update();

    let start = Instant::now();
    for _ in 0..num_steps {
        sim.update();
    }
    let elapsed = start.elapsed();

    let per_step = elapsed / num_steps as u32;
    println!(
        "N = {:>5}  |  total: {:>8.2?}  |  per step: {:>10.2?}",
        num_bodies, elapsed, per_step
    );
}

fn main() {
    let step_count = 5000;

    for &n in &[10, 50, 100, 200, 400, 800, 1600, 3200, 6400, 12800, 25600, 51200] {
        benchmark(n, step_count);
    }
}