mod gravity;
mod quadtree;
mod threads;

use crate::math::Vec2;
use crate::simulation::threads::{SimData, ThreadPool};
use crate::visualise::osk_graphics::Colour;

use std::collections::VecDeque;

pub const G: f64 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;

#[derive(Debug)]
pub struct Trail {
    pub points: VecDeque<Vec2>,
    pub max_length: usize,
}

#[derive(Debug)]
pub struct Body {
    pub mass: f64, // convention will be to give this in mutiples of solar mass (2 * 10^30kg)
    pub pos: Vec2, // convention will be to give legnth in AU
    pub vel: Vec2,
    pub trail: Trail,
    pub colour: Colour,
    pub radius: f64
}

#[derive(Copy, Clone)]
pub enum Potential {
    Gravity3d,
    Gravity2d,
}

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub dt: f64, // in years
    pub height: f64,
    pub length: f64,
    pub potential: Potential,
    thread_pool: ThreadPool
}

impl Trail {
    pub fn new(max_length: usize) -> Self {
        Trail { points: VecDeque::new(), max_length }
    }

    pub fn add_point(&mut self, point: Vec2) {
        self.points.push_back(point);
        if self.points.len() > self.max_length {
            self.points.pop_front();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec2> {
        self.points.iter()
    }
}

impl Body {
    pub fn new(mass: f64, pos: Vec2, vel: Vec2, colour: Colour, radius: f64, trail_length: usize) -> Body {
        Body { mass, pos, vel, trail: Trail::new(trail_length), colour, radius }
    }

    pub fn update(&mut self, force: Vec2, dt: f64) {
        self.trail.add_point(self.pos);

        self.vel += (dt / self.mass) * force;

        self.pos += dt * self.vel;
    }
}

impl Simulation {
    pub fn new(height: f64, length: f64, dt: f64, potential: Potential, num_threads: usize) -> Simulation {
        let thread_pool = ThreadPool::new(num_threads, SimData {
            height,
            length,
            dt,
            potential
        });

        Simulation { bodies: Vec::new(), dt, height, length, potential, thread_pool }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        match self.potential {
            Potential::Gravity3d | Potential::Gravity2d => {
                self.update_gravity();
            }
        }
    }

    pub fn zero_total_momentum(&mut self) {
        let total_momentum: Vec2 = self.bodies.iter()
            .map(|b| b.vel * b.mass)
            .fold(Vec2::new(0.0, 0.0), |acc, p| acc + p);

        let total_mass: f64 = self.bodies.iter().map(|b| b.mass).sum();

        let correction = total_momentum * (-1.0 / total_mass);

        // apply the correction to ONE body (conventionally the most massive/central one)
        if let Some(heaviest) = self.bodies.iter_mut().max_by(|a, b| a.mass.partial_cmp(&b.mass).unwrap()) {
            heaviest.vel += correction;
        }
    }
}

pub fn enforce_boundaries(body: &mut Body, length: f64, height: f64) {
    if body.pos.x < 0.0 {
        body.pos.x = 0.0;
    }
    else if body.pos.x > length {
        body.pos.x = length;
    }

    if body.pos.y < 0.0 {
        body.pos.y = 0.0;
    }
    else if body.pos.y > height {
        body.pos.y = height;
    }
}

