use crate::{simulation::Potential::Gravity3d, vector};
use vector::Vec2;
use std::collections::VecDeque;

use crate::visualise::osk_graphics::Colour;

const G: f64 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;

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

pub enum Potential {
    Gravity3d
}

pub struct Simulation {
    pub bodies: Vec<Body>,
    dt: f64, // in years
    pub height: f64,
    pub length: f64,
    pub potential: Potential,
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
    pub fn new(mass: f64, pos: Vec2, vel: Vec2, colour: Colour, radius: f64) -> Body {
        Body { mass, pos, vel, trail: Trail::new(1000), colour, radius }
    }

    pub fn update(&mut self, force: Vec2, dt: f64) {
        self.trail.add_point(self.pos);

        self.vel += (dt / self.mass) * force;

        self.pos += dt * self.vel;
    }
}

impl Simulation {
    pub fn new(height: f64, length: f64, dt: f64, potential: Potential) -> Simulation {
        Simulation { bodies: Vec::new(), dt, height, length, potential }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        match self.potential {
            Gravity3d => {
                self.update_gravity_3d();
            }
        }
    }

    fn update_gravity_3d(&mut self) {
        let forces: Vec<Vec2> = self.bodies.iter()
            .map(|body|self.compute_force(body))
            .collect();

        for (body, force) in self.bodies.iter_mut().zip(forces.iter()) {
            body.update(*force, self.dt);
        }
    }

    fn compute_force(&self, body: &Body) -> Vec2 {

        let mut force: Vec2 = Vec2::new(0.0, 0.0);
        for other_body in self.bodies.iter() {
            let dir: Vec2 = other_body.pos - body.pos;
            let mag: f64 = dir.magnitude_squared();
            if mag < 1e-9 {
                continue;
            }

            let norm: Vec2 = dir.normalised();

            force += (G * body.mass * other_body.mass / mag) * norm; 
        }

        force
    }
}

