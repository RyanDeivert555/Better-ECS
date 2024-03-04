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

        {
            let mut pos = world.get_component_mut::<(f32, f32)>(e1).unwrap();
            pos.0 += 10.0;
        }

        assert_eq!((110.0, 100.0), *world.get_component::<(f32, f32)>(e1).unwrap());
        assert_eq!(20.0, *world.get_component::<f32>(e2).unwrap());
        assert_eq!(None, world.get_component::<(f32, f32)>(e2).as_deref());

        world.remove_component::<f32>(e1);
        assert_eq!(None, world.get_component::<f32>(e1).as_deref());

        world.remove_entity(e1);
        assert_eq!(None, world.get_component::<(f32, f32)>(e1).as_deref());
    }

    #[test]
    fn multi_mut_borrow() {
        let mut world = World::new();

        world.register::<f32>();
        world.register::<(f32, f32)>();

        let id = world.new_entity().with(0.0).with((0.0, 0.0)).build();

        let mut value1 = world.get_component_mut::<f32>(id).unwrap();
        let mut value2 = world.get_component_mut::<(f32, f32)>(id).unwrap();

        *value1 = 1.0;
        value2.0 = 1.0;

        assert_eq!(*value1, 1.0);
        assert_eq!(*value2, (1.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn mut_borrow_error() {
        let mut world = World::new();

        world.register::<f32>();

        let e = world.new_entity().with(0.0).build();

        let _b1 = world.get_component::<f32>(e).unwrap();
        let mut b2 = world.get_component_mut::<f32>(e).unwrap();

        *b2 += 1.0;
    }
}
