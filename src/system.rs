use crate::world::World;

pub trait StartUpSystem {
    fn run(&self, world: &mut World) -> bool;
}

pub trait System {
    fn run(&self, world: &mut World) -> bool;
}
