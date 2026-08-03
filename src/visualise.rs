pub mod osk_graphics;
use osk_graphics::{Window, Point};

use crate::simulation;
use simulation::Simulation;

use crate::math::Vec2;

pub struct Visualiser {
    window: Window,
    sim: Simulation,
}

impl Visualiser {
    pub fn new(sim: Simulation, width: u32, height: u32) -> Option<Visualiser> {
        let win: Window = Window::open(width, height, "Simulation")?;

        win.set_background(osk_graphics::Colour::BLACK);
        Some(Visualiser { window: win, sim })
    }

    pub fn run(&mut self, timesteps: usize) {
        if !self.window.is_open {
            return;
        }

        let mut timestep: usize = 0;
        while self.window.poll_events() && timestep < timesteps {
            self.sim.update();

            self.window.begin_frame();

            for body in self.sim.bodies.iter() {
                let mut first_pos: Option<&Vec2> = None; 
                for (index, pos) in body.trail.iter().enumerate() {

                    let trail_length: usize = body.trail.points.len();
                    if let Some(start) = first_pos {
                        self.window.draw_line(
                            convert_sim_to_window_coords(self, *start),
                            convert_sim_to_window_coords(self, *pos), 
                            1.0,
                        osk_graphics::Colour { r: 1.0, g: 1.0, b: 1.0, a: index as f32 / (trail_length as f32) });
                    }
                    first_pos = Some(pos);
                }

                self.window.draw_circle(convert_sim_to_window_coords(self, body.pos), (body.radius * (self.window.height as f64 / self.sim.height)) as f32 , 15, body.colour);
            }

            self.window.end_frame();

            timestep += 1;
        }
    }
}

fn convert_sim_to_window_coords(vis: &Visualiser, sim_coord: Vec2) -> Point {
    let win_x: f32 = (sim_coord.x * (vis.window.width as f64 / vis.sim.length)) as f32;
    let win_y: f32 = ((vis.sim.height - sim_coord.y) * (vis.window.height as f64 / vis.sim.height)) as f32;

    Point {x: win_x, y: win_y}
}