#![allow(unused)]
use crate::{
    scheduler::{Scheduler, WorldFn},
    world::World,
};

#[derive(Default)]
pub struct App {
    world: World,
    scheduler: Scheduler,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn add_startup_system(&mut self, startup_system: WorldFn) -> &mut Self {
        self.scheduler.add_startup_system(startup_system);

        self
    }

    pub fn add_system(&mut self, system: WorldFn) -> &mut Self {
        self.scheduler.add_system(system);

        self
    }

    pub fn run(&mut self) {
        self.scheduler.run_startup_systems(&mut self.world);

        while self.world.active() {
            self.scheduler.run_systems(&mut self.world);
        }
    }
}
