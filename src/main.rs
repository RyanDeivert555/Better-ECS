mod component;
mod entity;
mod world;
use crate::{component::Component, world::World};

impl Component for (f32, f32) {}
impl Component for f32 {}

fn main() {
    let mut world = World::new();

    world.register::<f32>();
    world.register::<(f32, f32)>();

    let e1 = world.new_entity().with((100.0, 100.0)).with(50.0).build();
    let e2 = world.new_entity().with(20.0).build();

    assert_eq!(Some(&(100.0, 100.0)), world.get_component::<(f32, f32)>(e1));
    assert_eq!(Some(&50.0), world.get_component::<f32>(e1));

    assert_eq!(Some(&20.0), world.get_component::<f32>(e2));
    assert_eq!(None, world.get_component::<(f32, f32)>(e2));
}
