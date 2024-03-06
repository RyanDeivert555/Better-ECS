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

fn add_counter(world: &mut World) -> bool {
    world.add_resource(Counter(0));

    true
}

fn display_counter(world: &mut World) -> bool {
    let counter = world.get_resource::<Counter>().unwrap();
    println!("Counter: {}", counter.0);

    true
}

fn inc_counter(world: &mut World) -> bool {
    let mut counter = world.get_resource_mut::<Counter>().unwrap();
    counter.0 += 1;

    counter.0 < 10
}

fn main() {
    App::new()
        .add_startup_system(add_counter)
        .add_system(display_counter)
        .add_system(inc_counter)
        .run()
}
