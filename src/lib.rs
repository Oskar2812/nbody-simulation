pub mod math;
pub mod simulation;
pub mod visualise;

pub use math::{Complex, Vec2};
pub use simulation::{Simulation, Body};
pub use visualise::Visualiser;
pub use crate::visualise::osk_graphics::{Colour, Window};