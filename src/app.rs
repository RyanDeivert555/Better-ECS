#![allow(unused)]
use crate::{
    scheduler::{Scheduler, WorldFn},
    world::World,
};

/// Core application struct with the world and scheduler.
#[derive(Default)]
pub struct App {
    world: World,
    scheduler: Scheduler,
}

impl App {
    /// Creates a new `App` instance with a world and scheduler
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),
        }
    }

    /// Adds a startup system to the app. Order of insertion does matter; systems that are added first are run first.
    pub fn add_startup_system(&mut self, startup_system: WorldFn) -> &mut Self {
        self.scheduler.add_startup_system(startup_system);

        self
    }

    /// Adds a system to the app. Order of insertion does matter; systems that are added first are run first.
    pub fn add_system(&mut self, system: WorldFn) -> &mut Self {
        self.scheduler.add_system(system);

        self
    }

    fn run_commands(&mut self) {
        let mut command_queue = {
            let mut command_queue = self.world.get_commands();
            std::mem::take(&mut *command_queue) // deref coersion
        }; //we drop the refmut here, so it can't interfere with the `&mut` borrow we take on the next line
        command_queue.run_commands(&mut self.world);
    }

    /// Driver of `App`
    pub fn run(&mut self) {
        self.scheduler.run_startup_systems(&mut self.world);

        while self.world.active() {
            self.scheduler.run_systems(&mut self.world);
            self.run_commands();
        }
    }
}
