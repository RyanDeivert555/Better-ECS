use crate::{component::Component, make_component};

// shouldnt be done like this, just for testing
impl Component for i32 {}
impl Component for f32 {}

make_component! {
    #[derive(Debug, PartialEq)]
    struct Position {
        x: i32,
        y: i32,
    }
}

#[cfg(test)]
mod tests {
    use crate::{make_component, prelude::EntityId, tests::Position, world::World};

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

        assert_eq!(Some(50), world.remove_component::<i32>(e1));
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

        // borrow and mut borrow of same component
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
        world.register::<f32>();
        world.register::<Position>();

        for i in 0..50 {
            let _ = world
                .new_entity()
                .with(i)
                .with(Position { x: i, y: i })
                .build();

            let _ = world
                .new_entity()
                .with(i as f32)
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

        let q1 = world.query::<(i32, Position)>();
        let q2 = world.query_mut::<(f32, Position)>();

        // testing borrow mut of the storages
        for _ in std::iter::zip(q1, q2) {}
    }

    #[test]
    fn resources() {
        let mut world = World::new();

        assert_eq!(None, world.add_resource(0));
        assert_eq!(Some(0), world.add_resource(0));

        {
            let mut counter = world.get_resource_mut::<i32>().unwrap();
            *counter += 1;
        }

        assert_eq!(Some(&1), world.get_resource::<i32>().as_deref());
        assert_eq!(Some(1), world.add_resource(10));
        assert_eq!(None, world.get_resource::<Position>().as_deref());
    }

    #[test]
    fn rand_test() {
        let mut world = World::new();

        make_component! {
            pub struct Player;
        }

        world.register::<Position>();
        world.register::<Player>();

        let _ = world
            .new_entity()
            .with(Position { x: 0, y: 0 })
            .with(Player)
            .build();
        let _ = (0..10)
            .map(|_| world.new_entity().build())
            .collect::<Vec<_>>();

        let positions = world.query_mut::<Position>();

        for mut pos in positions {
            pos.x += 1;
        }

        {
            let (_, mut pos) = world.query_mut::<(Player, Position)>().next().unwrap();
            pos.x += 1;
        }

        let (_, pos) = world.query::<(Player, Position)>().next().unwrap();

        assert_eq!(*pos, Position { x: 2, y: 0 });
    }

    #[test]
    fn id_as_component() {
        let mut world = World::new();

        let e1 = world.new_entity().build();
        let e2 = world.new_entity().build();
        let e3 = world.new_entity().build();

        {
            let ids = world.query::<EntityId>().collect::<Vec<_>>();

            assert!(ids.iter().position(|cell| **cell == e1).is_some());
            assert!(ids.iter().position(|cell| **cell == e2).is_some());
            assert!(ids.iter().position(|cell| **cell == e3).is_some());
        }

        world.remove_entity(e1);

        {
            let ids = world.query::<EntityId>().collect::<Vec<_>>();
            assert!(ids.iter().position(|cell| **cell == e1).is_none());
            assert_eq!(None, world.get_component::<EntityId>(e1).as_deref());
            assert!(ids.iter().position(|cell| **cell == e2).is_some());
            assert!(ids.iter().position(|cell| **cell == e3).is_some());
        }
    }
}
