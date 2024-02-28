use crate::component::Component;
impl Component for (f32, f32) {}
impl Component for f32 {}

#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn basic_operations() {
        let mut world = World::new();

        world.register::<f32>();
        world.register::<(f32, f32)>();

        let e1 = world.new_entity().with((100.0, 100.0)).with(50.0).build();
        let e2 = world.new_entity().with(20.0).build();

        let mut pos = world.get_component_mut::<(f32, f32)>(e1).unwrap();
        pos.0 += 10.0;
    }

    #[test]
    fn multi_mut_borrow() {
        let mut world = World::new();

        world.register::<f32>();
        world.register::<(f32, f32)>();

        let id = world.new_entity().with(0.0).with((0.0, 0.0)).build();

        let value1 = world.get_component_mut::<f32>(id).unwrap();
        let value2 = world.get_component_mut::<(f32, f32)>(id).unwrap();

        *value1 = 1.0;
    }
}
