use crate::vector;
use vector::Vec2;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Trail {
    pub points: VecDeque<Vec2>,
    pub max_length: usize,
}

#[derive(Debug)]
pub struct Body {
    pub mass: f64, // convention will be to give this in mutiples of Earths mass (5.972 * 10^24kg)
    pub pos: Vec2, // convention will be to give legnth in AU
    pub vel: Vec2,
    pub trail: Trail,
}

pub struct Simulation {
    pub bodies: Vec<Body>,
    dt: f64,
    pub height: f64,
    pub length: f64
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
    pub fn new(mass: f64, pos: Vec2, vel: Vec2) -> Body {
        Body { mass, pos, vel, trail: Trail::new(1000) }
    }

    pub fn update(&mut self, dt: f64) {
        self.trail.add_point(self.pos);

        self.pos += dt * self.vel;
    }
}

impl Simulation {
    pub fn new(height: f64, length: f64, dt: f64) -> Simulation {
        Simulation { bodies: Vec::new(), dt, height, length }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        for body in self.bodies.iter_mut() {
            body.update(self.dt);
        }
    }
}

