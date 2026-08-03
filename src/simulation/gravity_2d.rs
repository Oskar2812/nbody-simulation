use crate::simulation::{Simulation, Body};
use crate::math::Vec2;

const MAX_DEPTH: usize = 10;
const MAX_BODIES_PER_LEAF: usize = 10;
const THETA: f64 = 0.5;

pub(crate) struct QuadNode {
    half_length: f64,
    children: Option<Box<[QuadNode; 4]>>,
    body_indexes: Vec<usize>, 
    center_of_mass: Option<Vec2>,
    total_mass: Option<f64>,
}

fn enforce_boundaries(body: &mut Body, length: f64, height: f64) {
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

impl Simulation {
    pub(crate) fn update_gravity_2d(&mut self) {
        let mut quad_tree: QuadNode = QuadNode::build_tree(
            &self.bodies,
            (0..self.bodies.len()).collect(),
            Vec2::new(self.length / 2.0, self.height / 2.0),
            self.length / 2.0,
            0);

            quad_tree.build_mass_distribution(&self.bodies);

        let forces: Vec<Vec2> = self.bodies.iter()
            .map(|body|self.compute_force_gravity_2d(body, &quad_tree))
            .collect();

        for (body, force) in self.bodies.iter_mut().zip(forces.iter()) {
            body.update(*force, self.dt);
            enforce_boundaries(body, self.length, self.height);
        }
    }

    fn compute_force_gravity_2d(&self, body: &Body, quad_tree: &QuadNode) -> Vec2 {
        compute_force_from_node(quad_tree, &self.bodies, body)
    }
}

impl QuadNode {
    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    fn new_leaf(half_length: f64, body_indexes: Vec<usize>) -> QuadNode {
        QuadNode { half_length, children: None, body_indexes, center_of_mass: None, total_mass: None }
    }

    fn new_branch(half_length: f64, children: [QuadNode; 4]) -> QuadNode {
        QuadNode { half_length, children: Some(Box::new(children)), body_indexes: Vec::new(), center_of_mass: None, total_mass: None }
    }

    fn build_tree(bodies: &[Body], body_indexes: Vec<usize>, center: Vec2, half_length: f64, depth: usize) -> QuadNode {
        if body_indexes.len() <= MAX_BODIES_PER_LEAF || depth >= MAX_DEPTH {
            return QuadNode::new_leaf(half_length, body_indexes);
        }

        let mut quadrant_indexes: [Vec<usize>; 4] = Default::default();
        for &i in &body_indexes {
            let q: u8 = quadrant(center, bodies[i].pos);
            quadrant_indexes[q as usize].push(i);
        }

        let child_half_length: f64 = half_length / 2.0;
        let offsets = [Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0), Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0)];

        let children: [QuadNode; 4] = std::array::from_fn(
            |q| {
                let child_center = center + offsets[q] * child_half_length;
                QuadNode::build_tree(bodies, quadrant_indexes[q].clone(), child_center, child_half_length, depth + 1)
            }
        );

        QuadNode::new_branch(half_length, children)
    }

    fn build_mass_distribution(&mut self, bodies: &[Body]) {
        let mut mass: f64 = 0.0;
        let mut com: Vec2 = Vec2::new(0.0, 0.0);
        
        if self.is_leaf() {
            for &i in &self.body_indexes {
                mass += bodies[i].mass;
                com += bodies[i].mass * bodies[i].pos;
            }

            if mass > 1e-9 {
                com = (1.0 / mass) * com;
            }
        
            self.total_mass = Some(mass);
            self.center_of_mass = Some(com);

            return;
        }

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                child.build_mass_distribution(bodies);

                let child_mass = child.total_mass.unwrap();
                let child_com = child.center_of_mass.unwrap();

                mass += child_mass;
                com += child_mass * child_com;
            }
        }

        if mass > 1e-9 {
            com = (1.0 / mass) * com;
        }

        self.total_mass = Some(mass);
        self.center_of_mass = Some(com);
       
    }
}

fn quadrant(center: Vec2, body_pos: Vec2) -> u8 {
    match (body_pos.x >= center.x, body_pos.y >= center.y) {
        (true, true) => 0,
        (false, true) => 1,
        (false, false) => 2,
        (true, false) => 3
    }
}

fn compute_force_from_node(node: &QuadNode, bodies: &[Body], target: &Body) -> Vec2 {
    let mut force = Vec2::new(0.0, 0.0);

    if node.is_leaf() {
        for &i in &node.body_indexes {
            let other_body: &Body = &bodies[i];
            let dir: Vec2 = other_body.pos - target.pos;
            let mag: f64 = dir.magnitude_squared();
            if mag < 1e-9 {
                continue;
            }

            force += (target.mass * other_body.mass / mag) * dir;
        }

        return force;
    }

    let dir = node.center_of_mass.unwrap() - target.pos;
    let distance = dir.magnitude();

    if distance < 1e-9 {
        return Vec2::new(0.0, 0.0);
    }

    if (node.half_length * 2.0) / distance < THETA {
        return (target.mass * node.total_mass.unwrap()) / (distance * distance) * dir;
    }

    if let Some(children) = &node.children {
        for child in children.iter() {
            force += compute_force_from_node(child, bodies, target);
        }
    }
    force
}