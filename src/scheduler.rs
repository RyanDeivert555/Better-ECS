#![allow(unused)]
use crate::world::World;

pub type WorldFn = fn(&mut World);

#[derive(Default)]
pub struct Scheduler {
    startup_systems: Vec<WorldFn>,
    // TODO: perf test to see if this is better than Fn trait
    systems: Vec<WorldFn>,
}

/// A scheduler used to manage and execute systems within the ECS.
impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a startup system to the app. Order of insertion does matter; systems that are added first are run first. 
    pub fn add_startup_system(&mut self, startup_system: WorldFn) {
        self.startup_systems.push(startup_system);
    }

    /// Adds a system to the app. Order of insertion does matter; systems that are added first are run first. 
    pub fn add_system(&mut self, system: WorldFn) {
        self.systems.push(system);
    }

    /// Runs all startup systems.
    pub fn run_startup_systems(&mut self, world: &mut World) {
        for system in self.startup_systems.iter_mut() {
            system(world);
        }
    }

    /// Runs all systems.
    pub fn run_systems(&mut self, world: &mut World) {
        for system in self.systems.iter_mut() {
            system(world);
        }
    }
}
