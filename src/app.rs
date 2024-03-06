#![allow(unused)]
use crate::{
    world::World,
    scheduler::Scheduler,
    system::{StartUpSystem, System},
};

pub struct App {
    world: World,
    scheduler: Scheduler,
    run: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),
            run: true,
        }
    }

    pub fn add_startup_system(&mut self, startup_system: impl StartUpSystem + 'static) -> &mut Self {
        self.scheduler.add_startup_system(startup_system);

        self
    }

    pub fn add_system(&mut self, system: impl System + 'static) -> &mut Self {
        self.scheduler.add_system(system);

        self
    }

    pub fn run(&mut self) {
        self.run = self.scheduler.run_startup_systems(&mut self.world);

        while self.run {
            self.run = self.scheduler.run_systems(&mut self.world);
        }
    }
}