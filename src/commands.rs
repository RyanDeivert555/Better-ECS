use crate::{component::Component, world::World};

type Command = Box<dyn FnMut(&mut World)>;

#[derive(Default)]
pub struct Commands {
    commands: Vec<Command>,
}
impl Component for Commands {}

impl Commands {
    pub fn add_command(&mut self, command: impl FnMut(&mut World) + 'static) {
        self.commands.push(Box::new(command));
    }

    pub fn run_commands(&mut self, world: &mut World) {
        // no need to consume since we use std::take
        for command in self.commands.iter_mut() {
            command(world);
        }
    }
}
