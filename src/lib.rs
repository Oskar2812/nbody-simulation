pub mod simulation;
pub mod vector;
pub mod visualise;

pub use vector::Vec2;
pub use simulation::{Simulation, Body};
pub use visualise::Visualiser;
pub use crate::visualise::osk_graphics::{Colour, Window};