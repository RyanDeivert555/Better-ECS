#![allow(unused)]
use crate::component::{Component,EntityId};
use crate::world::World;

pub struct EntityBuilder<'a> {
    id: EntityId,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(id: EntityId, world: &'a mut World) -> Self {
        world.add_component(id, id);

        Self { id, world }
    }

    pub fn with<T>(&mut self, entry: T) -> &mut Self
    where
        T: Component + 'static,
    {
        self.world.add_component(self.id, entry);

        self
    }

    pub fn build(&self) -> EntityId {
        self.id
    }
}
