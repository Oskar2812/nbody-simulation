use crate::simulation::Potential::{Gravity2d, Gravity3d};
use crate::simulation::{Simulation, Body, enforce_boundaries, G};
use crate::simulation::quadtree::{QuadNode, compute_force_from_node, THETA};
use crate::math::Vec2;

impl Simulation {
    pub(crate) fn update_gravity(&mut self) {
        let mut quad_tree: QuadNode = QuadNode::build_tree(
            &self.bodies,
            (0..self.bodies.len()).collect(),
            Vec2::new(self.length / 2.0, self.height / 2.0),
            self.length / 2.0,
            0);

            quad_tree.build_mass_distribution(&self.bodies);

        let forces: Vec<Vec2> = self.bodies.iter()
            .map(|body|self.compute_force_from_node(&quad_tree, &self.bodies, body))
            .collect();

        for (body, force) in self.bodies.iter_mut().zip(forces.iter()) {
            body.update(*force, self.dt);
            enforce_boundaries(body, self.length, self.height);
        }
    }

    pub(crate) fn compute_force_from_node(&self, node: &QuadNode, bodies: &[Body], target: &Body) -> Vec2 {
    let mut force = Vec2::new(0.0, 0.0);

    if node.is_leaf() {
        for &i in &node.body_indexes {
            let other_body: &Body = &bodies[i];
            let dir: Vec2 = other_body.pos - target.pos;
            let mag: f64 = dir.magnitude_squared();
            if mag < 1e-9 {
                continue;
            }

            match self.potential {
                Gravity3d => {
                    force += G * (target.mass * other_body.mass / mag) * dir.normalised();
                }
                Gravity2d => {
                    force += (target.mass * other_body.mass / mag) * dir;
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
        match self.potential {
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
            force += compute_force_from_node(child, bodies, target);
        }
    }
    force
}
}