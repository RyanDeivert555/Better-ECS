#![allow(unused)]
use crate::component::{Component,EntityId};
use crate::world::World;

pub struct EntityBuilder<'a> {
    id: EntityId,
    world: &'a mut World,
}

/// An entity builder used to create and configure entities within a `World`.
impl<'a> EntityBuilder<'a> {
    pub fn new(id: EntityId, world: &'a mut World) -> Self {
        world.add_component(id, id);

        Self { id, world }
    }

    /// Adds a component to the entity being built.
    pub fn with<T>(&mut self, entry: T) -> &mut Self
    where
        T: Component + 'static,
    {
        self.world.add_component(self.id, entry);

        self
    }

    /// Returns the new entity's ID and drops the reference to `World`.
    pub fn build(&self) -> EntityId {
        self.id
    }
}
