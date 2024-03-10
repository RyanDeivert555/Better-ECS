mod app;
mod component;
mod entity_builder;
mod prelude;
mod query;
mod scheduler;
mod tests;
mod world;
use crate::prelude::*;

make_component! {
    struct Counter(usize);
}

fn add_counter(world: &mut World) {
    world.add_resource(Counter(0));
}

fn display_counter(world: &mut World) {
    let counter = world.get_resource::<Counter>().unwrap();
    println!("Counter: {}", counter.0);
}

fn inc_counter(world: &mut World) {
    let counter = {
        let mut counter = world.get_resource_mut::<Counter>().unwrap();
        counter.0 += 1;

        counter.0
    };

    if counter >= 10 {
        world.shutdown();
    }
}

fn main() {
    App::new()
        .add_startup_system(add_counter)
        .add_system(display_counter)
        .add_system(inc_counter)
        .run()
}
