use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use crate::Simulation;
use crate::simulation::{Potential, Body, enforce_boundaries};
use crate::simulation::quadtree::QuadNode;
use crate::math::Vec2;

struct WorkData {
    bodies: *mut Body,
    bodies_len: usize,
    quad_tree: Arc<QuadNode>,
    bodies_mass: Arc<Vec<f64>>,
    bodies_pos: Arc<Vec<Vec2>>
}

unsafe impl Send for WorkData {}

pub struct SimData {
    pub height: f64,
    pub length: f64,
    pub dt: f64,
    pub potential: Potential
}

// Handoff slot for one worker: the dispatcher writes `data` then publishes it by
// bumping `generation` (Release); the worker spins on `generation` (Acquire) instead
// of blocking on a channel/condvar, since the OS park/wake round trip is far more
// expensive than the per-step work being handed off. `done` is the mirror signal
// back to the dispatcher, also via spin, once the worker finishes its chunk.
struct WorkerSlot {
    generation: AtomicUsize,
    done: AtomicUsize,
    data: UnsafeCell<Option<WorkData>>,
}

// Manual synchronisation via the generation/done atomics (Acquire/Release) takes the
// place of the Sync guarantee UnsafeCell can't provide on its own.
unsafe impl Sync for WorkerSlot {}

impl WorkerSlot {
    fn new() -> Self {
        WorkerSlot { generation: AtomicUsize::new(0), done: AtomicUsize::new(0), data: UnsafeCell::new(None) }
    }
}

// Sentinel published in place of a real generation number to tell a worker to exit
// its spin loop and return, instead of spinning forever after the pool is dropped.
const SHUTDOWN: usize = usize::MAX;

pub struct ThreadPool {
    slots: Vec<Arc<WorkerSlot>>,
    generation: usize,
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: usize, sim_data: SimData) -> Self {
        let mut slots = Vec::with_capacity(num_threads);
        let mut workers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let slot = Arc::new(WorkerSlot::new());
            slots.push(Arc::clone(&slot));

            let height = sim_data.height;
            let length = sim_data.length;
            let dt = sim_data.dt;
            let potential = sim_data.potential;

            workers.push(thread::spawn(move || {
                let mut seen_generation = 0usize;

                loop {
                    while slot.generation.load(Ordering::Acquire) == seen_generation {
                        std::hint::spin_loop();
                    }
                    seen_generation = slot.generation.load(Ordering::Acquire);

                    if seen_generation == SHUTDOWN {
                        return;
                    }

                    let data = unsafe { (*slot.data.get()).take() }
                        .expect("worker woken with no work queued");

                    let chunk = unsafe {
                        std::slice::from_raw_parts_mut(data.bodies, data.bodies_len)
                    };

                    let forces: Vec<Vec2> = chunk.iter()
                        .map(|body| Simulation::compute_force_from_node(&data.quad_tree, &data.bodies_pos, &data.bodies_mass, body, potential))
                        .collect();

                    for (body, force) in chunk.iter_mut().zip(forces.iter()) {
                        body.update(*force, dt);
                        enforce_boundaries(body, length, height);
                    }

                    slot.done.store(seen_generation, Ordering::Release);
                }
            }));
        }

        ThreadPool { slots, generation: 0, workers }
    }

    pub fn run(
        &mut self,
        bodies: &mut [Body],
        quad_tree: &Arc<QuadNode>,
        bodies_mass: &Arc<Vec<f64>>,
        bodies_pos: &Arc<Vec<Vec2>>
    ) {
        self.generation += 1;
        let generation = self.generation;

        let chunk_size = bodies.len().div_ceil(self.slots.len()).max(1);
        let mut chunks = bodies.chunks_mut(chunk_size);

        for slot in self.slots.iter() {
            let chunk = chunks.next().unwrap_or(&mut []);
            let work = WorkData {
                bodies: chunk.as_mut_ptr(),
                bodies_len: chunk.len(),
                quad_tree: Arc::clone(quad_tree),
                bodies_mass: Arc::clone(bodies_mass),
                bodies_pos: Arc::clone(bodies_pos)
            };

            unsafe { *slot.data.get() = Some(work); }
            slot.generation.store(generation, Ordering::Release);
        }

        for slot in self.slots.iter() {
            while slot.done.load(Ordering::Acquire) != generation {
                std::hint::spin_loop();
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for slot in &self.slots {
            slot.generation.store(SHUTDOWN, Ordering::Release);
        }

        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}
