pub mod osk_graphics;
use osk_graphics::{Colour, Window, Point};

use crate::simulation;
use simulation::Simulation;

use crate::math::Vec2;

use std::sync::mpsc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct FrameData {
    bodies: Vec<BodyDrawInfo>,
}

pub struct BodyDrawInfo {
    pos: Vec2,
    radius: f64,
    colour: Colour,
    trail: Vec<Vec2>,   // just the positions needed for drawing, not the whole Trail/Body
}

pub struct Visualiser {
    window: Window,
    sim: Option<Simulation>,
}

impl Visualiser {
    pub fn new(sim: Simulation, width: u32, height: u32) -> Option<Visualiser> {
        let win: Window = Window::open(width, height, "Simulation")?;

        win.set_background(osk_graphics::Colour::BLACK);
        Some(Visualiser { window: win, sim: Some(sim) })
    }

    pub fn run(mut self, timesteps: usize) -> Visualiser{
        if !self.window.is_open {
            return self;
        }

        let mut timestep: usize = 0;

        let (sender, receiver) = mpsc::channel::<FrameData>();

        let mut sim = self.sim.take().expect("Simulation missing");
        let sim_height = sim.height;
        let sim_length = sim.length;

        let stop_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let stop_flag_clone: Arc<AtomicBool> = Arc::clone(&stop_flag);
        let sim_handle = thread::spawn(move || {
            for _ in 0..timesteps {
                if stop_flag_clone.load(Ordering::Relaxed) {
                    return sim;
                }
                sim.update();

                let frame = FrameData {
                    bodies: sim.bodies.iter().map(|b| BodyDrawInfo {
                        pos: b.pos,
                        radius: b.radius,
                        colour: b.colour,
                        trail: b.trail.points.iter().cloned().collect(),
                    }).collect(),
                };

                if sender.send(frame).is_err() {
                    break;
                }
            }

            sim
        });

        while self.window.poll_events() && timestep < timesteps {
            if let Ok(frame) = receiver.recv() {
                self.window.begin_frame();

                for body in &frame.bodies {
                    let mut first_pos: Option<&Vec2> = None;
                    let trail_length = body.trail.len();

                    for (index, pos) in body.trail.iter().enumerate() {
                        if let Some(start) = first_pos {
                            self.window.draw_line(
                                convert_sim_to_window_coords(self.window.width, self.window.height, sim_length, sim_height, *start),
                                convert_sim_to_window_coords(self.window.width, self.window.height, sim_length, sim_height, *pos),
                                1.0,
                                Colour { r: 1.0, g: 1.0, b: 1.0, a: index as f32 / trail_length as f32 },
                            );
                        }
                        first_pos = Some(pos);
                    }

                    self.window.draw_circle(
                        convert_sim_to_window_coords(self.window.width, self.window.height, sim_length, sim_height, body.pos),
                        (body.radius * (self.window.height as f64 / sim_height)) as f32,
                        15,
                        body.colour,
                    );
                }

                self.window.end_frame();
            }

            timestep += 1;
        }

        stop_flag.store(true, Ordering::Relaxed);

        let sim = sim_handle.join().unwrap();   // reclaim ownership once the sim thread finishes
        self.sim = Some(sim);

        self
    }
}

fn convert_sim_to_window_coords(win_width: u32, win_height: u32, sim_length: f64, sim_height: f64, sim_coord: Vec2) -> Point {
    let win_x: f32 = (sim_coord.x * (win_width as f64 / sim_length)) as f32;
    let win_y: f32 = ((sim_height - sim_coord.y) * (win_height as f64 / sim_height)) as f32;

    Point {x: win_x, y: win_y}
}