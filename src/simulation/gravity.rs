use crate::simulation::Potential::{self, Gravity2d, Gravity3d};
use crate::simulation::{Simulation, Body, G};
use crate::simulation::quadtree::{QuadNode, THETA};
use crate::math::Vec2;

use std::sync::Arc;

impl Simulation {
    pub(crate) fn update_gravity(&mut self) {
        let mut quad_tree: QuadNode = QuadNode::build_tree(
            &self.bodies,
            (0..self.bodies.len()).collect(),
            Vec2::new(self.length / 2.0, self.height / 2.0),
            self.length / 2.0,
            0);

        quad_tree.build_mass_distribution(&self.bodies);

        let quad_tree_arc = Arc::new(quad_tree);
        let bodies_pos: Arc<Vec<Vec2>> = Arc::new(self.bodies.iter().map(|body| body.pos).collect());
        let bodies_mass: Arc<Vec<f64>> = Arc::new(self.bodies.iter().map(|body| body.mass).collect());

        self.thread_pool.run(self.bodies.as_mut_slice(), &quad_tree_arc, &bodies_mass, &bodies_pos);
    }

    pub(crate) fn compute_force_from_node(node: &QuadNode, bodies: &[Vec2], bodies_mass: &[f64], target: &Body, potential: Potential) -> Vec2 {
        let mut force = Vec2::new(0.0, 0.0);

        if node.is_leaf() {
            for &i in &node.body_indexes {
                let dir: Vec2 = bodies[i] - target.pos;
                let mag: f64 = dir.magnitude_squared();
                if mag < 1e-9 {
                    continue;
                }

                match potential {
                    Gravity3d => {
                        force += G * (target.mass * bodies_mass[i] / mag) * dir.normalised();
                    }
                    Gravity2d => {
                        force += (target.mass * bodies_mass[i] / mag) * dir;
                    }
                }
            }
            return force;
        }

        let dir = node.center_of_mass.unwrap() - target.pos;
        let distance = dir.magnitude();

        if distance < 1e-9 {
            return Vec2::new(0.0, 0.0);
        }

        if (node.half_length * 2.0) / distance < THETA {
            match potential {
                Gravity3d => {
                    return G * (target.mass * node.total_mass.unwrap()) / (distance * distance) * dir.normalised();
                }
                Gravity2d => {
                    return (target.mass * node.total_mass.unwrap()) / (distance * distance) * dir;
                }
            }

        }

        if let Some(children) = &node.children {
            for child in children.iter() {
                force += Self::compute_force_from_node(child, bodies, bodies_mass, target, potential);
            }
        }
        force
    }
}