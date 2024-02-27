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

        assert_eq!(Some(&(100.0, 100.0)), world.get_component::<(f32, f32)>(e1));
        assert_eq!(Some(&50.0), world.get_component::<f32>(e1));

        assert_eq!(Some(&20.0), world.get_component::<f32>(e2));
        assert_eq!(None, world.get_component::<(f32, f32)>(e2));
        world.remove_component::<f32>(e2);
        assert_eq!(None, world.get_component::<f32>(e2));

        let pos = world.get_component_mut::<(f32, f32)>(e1).unwrap();
        pos.0 += 10.0;
        assert_eq!(Some(&(110.0, 100.0)), world.get_component::<(f32, f32)>(e1));
    }

    #[test]
    fn queries() {
        let mut world = World::new();

        world.register::<f32>();
        world.register::<(f32, f32)>();

        let e1 = world.new_entity().with((100.0, 100.0)).with(50.0).build();
        let e2 = world.new_entity().with(20.0).build();

        let query1 = world.query::<f32, (f32, f32)>();
        let query2 = world.query_mut::<f32, (f32, f32)>();
        assert_eq!(query2.count(), 1);
    }
}
