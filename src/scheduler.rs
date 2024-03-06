#![allow(unused)]
use crate::world::World;

type BoxedVec<T> = Vec<Box<T>>;

#[derive(Default)]
pub struct Scheduler {
    startup_systems: BoxedVec<dyn Fn(&mut World) -> bool>,
    systems: BoxedVec<dyn Fn(&mut World) -> bool>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_startup_system<F>(&mut self, startup_system: F) 
    where
        F: Fn(&mut World) -> bool + 'static,
    {
        self.startup_systems.push(Box::new(startup_system));
    }

    pub fn add_system<F>(&mut self, system: F)
    where
        F: Fn(&mut World) -> bool + 'static,
    {
        self.systems.push(Box::new(system));
    }

    pub fn run_startup_systems(&mut self, world: &mut World) -> bool {
        for system in self.startup_systems.iter_mut() {
            if !system(world) {
                return false;
            }
        }

        true
    }

    pub fn run_systems(&mut self, world: &mut World) -> bool {
        for system in self.systems.iter_mut() {
            if !system(world) {
                return false;
            }
        }

        true
    }
}