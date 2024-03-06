mod app;
mod component;
mod entity_builder;
mod prelude;
mod query;
mod scheduler;
mod system;
mod tests;
mod world;
use crate::prelude::*;

make_component! {
    struct Counter(usize);
}

struct AddCounter;
impl StartUpSystem for AddCounter {
    fn run(&self, world: &mut World) -> bool {
        world.add_resource(Counter(0));

        true
    }
}

struct DisplayCount;
impl System for DisplayCount {
    fn run(&self, world: &mut World) -> bool {
        let counter = world.get_resource::<Counter>().unwrap();
        println!("Counter: {}", counter.0);

        true
    }
}

struct IncCounter;
impl System for IncCounter {
    fn run(&self, world: &mut World) -> bool {
        let mut counter = world.get_resource_mut::<Counter>().unwrap();
        counter.0 += 1;

        counter.0 < 10
    }
}

fn main() {
    App::new()
        .add_startup_system(AddCounter)
        .add_system(DisplayCount)
        .add_system(IncCounter)
        .run()
}
