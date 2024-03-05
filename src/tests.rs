use crate::{component::Component, make_component};

// shouldnt be done like this, just for testing
impl Component for i32 {}

make_component! {
    #[derive(Debug, PartialEq)]
    struct Position {
        x: i32,
        y: i32,
    }
}

#[cfg(test)]
mod tests {
    use crate::{tests::Position, world::World};

    #[test]
    fn basic_operations() {
        let mut world = World::new();

        world.register::<i32>();
        world.register::<Position>();

        let e1 = world
            .new_entity()
            .with(Position { x: 100, y: 100 })
            .with(50)
            .build();
        let e2 = world.new_entity().with(20).build();

        {
            let mut pos = world.get_component_mut::<Position>(e1).unwrap();
            pos.x += 10;
        }

        assert_eq!(
            Position { x: 110, y: 100 },
            *world.get_component::<Position>(e1).unwrap()
        );
        assert_eq!(20, *world.get_component::<i32>(e2).unwrap());
        assert_eq!(None, world.get_component::<Position>(e2).as_deref());

        world.remove_component::<i32>(e1);
        assert_eq!(None, world.get_component::<i32>(e1).as_deref());

        world.remove_entity(e1);
        assert_eq!(None, world.get_component::<Position>(e1).as_deref());
    }

    #[test]
    fn multi_mut_borrow() {
        let mut world = World::new();

        world.register::<i32>();
        world.register::<Position>();

        let id = world
            .new_entity()
            .with(0)
            .with(Position { x: 0, y: 0 })
            .build();

        let mut value1 = world.get_component_mut::<i32>(id).unwrap();
        let mut value2 = world.get_component_mut::<Position>(id).unwrap();

        *value1 = 1;
        value2.x = 1;

        assert_eq!(*value1, 1);
        assert_eq!(*value2, Position { x: 1, y: 0 });
    }

    #[test]
    #[should_panic]
    fn mut_borrow_error() {
        let mut world = World::new();

        world.register::<i32>();

        let e = world.new_entity().with(0).build();

        let _b1 = world.get_component::<i32>(e).unwrap();
        let mut b2 = world.get_component_mut::<i32>(e).unwrap();

        *b2 += 1;
    }

    #[test]
    fn queries1() {
        let mut world = World::new();

        world.register::<i32>();
        world.register::<Position>();

        for i in 0..50 {
            let _ = world
                .new_entity()
                .with(i)
                .with(Position { x: i, y: i })
                .build();
        }

        let q1 = world.query::<(i32, Position)>();

        for (i, (num, pos)) in q1.enumerate() {
            assert_eq!(i as i32, *num);
            assert_eq!(
                *pos,
                Position {
                    x: i as i32,
                    y: i as i32
                }
            );
        }
    }

    #[test]
    fn queries2() {
        let mut world = World::new();

        world.register::<i32>();
        world.register::<Position>();

        for i in 0..50 {
            let _ = world
                .new_entity()
                .with(i)
                .with(Position { x: i, y: i })
                .build();
        }

        let q2 = world.query_mut::<(i32, Position)>();
        for (num, mut pos) in q2 {
            if *num % 2 == 0 {
                pos.x += 1;
                pos.y -= 1;
            }
        }

        let q2 = world.query::<(i32, Position)>();
        for (i, (num, pos)) in q2.enumerate() {
            let expected_pos = {
                if *num % 2 == 0 {
                    Position {
                        x: i as i32 + 1,
                        y: i as i32 - 1,
                    }
                } else {
                    Position {
                        x: i as i32,
                        y: i as i32,
                    }
                }
            };
            assert_eq!(i as i32, *num);
            assert_eq!(expected_pos, *pos);
        }
    }
}
