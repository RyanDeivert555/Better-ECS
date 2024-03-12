#![allow(unused)]
use crate::world::World;

pub type WorldFn = fn(&mut World);

#[derive(Default)]
pub struct Scheduler {
    startup_systems: Vec<WorldFn>,
    // TODO: perf test to see if this is better than Fn trait
    systems: Vec<WorldFn>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_startup_system(&mut self, startup_system: WorldFn) {
        self.startup_systems.push(startup_system);
    }

    pub fn add_system(&mut self, system: WorldFn) {
        self.systems.push(system);
    }

    pub fn run_startup_systems(&mut self, world: &mut World) {
        for system in self.startup_systems.iter_mut() {
            system(world);
        }
    }

    pub fn run_systems(&mut self, world: &mut World) {
        for system in self.systems.iter_mut() {
            system(world);
        }
    }
}
