#![allow(unused)]
use crate::{
    world::World,
    system::{StartUpSystem, System}
};

#[derive(Default)]
pub struct Scheduler {
    startup_systems: Vec<Box<dyn StartUpSystem>>,
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_startup_system(&mut self, startup_system: impl StartUpSystem + 'static) {
        self.startup_systems.push(Box::new(startup_system));
    }

    pub fn add_system(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }

    pub fn run_startup_systems(&mut self, world: &mut World) -> bool {
        for system in self.startup_systems.iter_mut() {
            if !system.run(world) {
                return false;
            }
        }

        true
    }

    pub fn run_systems(&mut self, world: &mut World) -> bool {
        for system in self.systems.iter_mut() {
            if !system.run(world) {
                return false;
            }
        }

        true
    }
}