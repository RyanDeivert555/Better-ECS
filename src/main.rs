mod component;
use component::*;
use slotmap::SlotMap;

impl Component for (f32, f32) {}
impl Component for f32 {}

#[derive(Default)]
struct World {
    positions: ComponentStorage<(f32, f32)>,
    healths: ComponentStorage<f32>,
    ids: SlotMap<EntityId, ()>,
}

struct EntityBuilder<'a> {
    id: EntityId,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(id: EntityId, world: &'a mut World) -> Self {
        Self {
            id,
            world,
        }
    }

    pub fn add_position(&mut self, position: (f32, f32)) -> &mut Self {
        self.world.add_position(self.id, position);

        self
    }

    pub fn add_health(&mut self, health: f32) -> &mut Self {
        self.world.add_health(self.id, health);

        self
    }

    pub fn build(&self) -> EntityId {
        self.id
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        let key = self.ids.insert(());

        EntityBuilder::new(key, self)
    }

    pub fn remove_entity(&mut self, key: EntityId) {
        self.positions.remove(key);
        self.healths.remove(key);
        self.ids.remove(key);
    }

    pub fn add_position(&mut self, key: EntityId, pos: (f32, f32)) {
        self.positions.insert(key, pos);
    }

    pub fn add_health(&mut self, key: EntityId, health: f32) {
        self.healths.insert(key, health);
    }

    pub fn position(&self, key: EntityId) -> (f32, f32) {
        self.positions.get(key).unwrap().clone()
    }

    pub fn health(&self, key: EntityId) -> f32 {
        self.healths.get(key).unwrap().clone()
    }

    pub fn query_health(&self) -> impl Iterator<Item = &f32> {
        self.healths.query()
    }
}

fn main() {
    let mut world = World::new();

    let e1 = world.new_entity()
        .add_health(100.0)
        .add_position((50.0, 10.0))
        .build();

    world.add_position(e1, (10.0, 50.0));

    let e2 = world.new_entity()
        .add_health(50.0)
        .add_position((100.0, -100.0))
        .build();

    assert_eq!(world.position(e1), (10.0, 50.0));
    assert_eq!(world.health(e1), 100.0);

    assert_eq!(world.position(e2), (100.0, -100.0));
    assert_eq!(world.health(e2), 50.0);

    for h in world.query_health() {
        println!("{:}", h);
    }
}
