#![allow(unused)]
use crate::{
    commands::Commands,
    entity_builder::EntityBuilder,
    component::{Component, ComponentStorage, EntityId},
    query::Query,
};
use slotmap::HopSlotMap;
use std::process::{Command, Output};
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

type ComponentMap = HashMap<TypeId, Box<dyn Any>>;
type ResourceMap = HashMap<TypeId, RefCell<Box<dyn Any>>>;

#[derive(Default)]
pub struct World {
    // has ComponentStorage<T>
    components: ComponentMap,
    // has resources T
    resources: ResourceMap,
    // TODO: test whether SlotMap or HopSlotMap is faster
    ids: HopSlotMap<EntityId, ()>,
    active: bool,
}

/// The core entity-component world representation.
/// This struct manages components, resources, entities, and their relationships.
impl World {
    /// Creates a new `World` instance.
    pub fn new() -> Self {
        let mut instance = Self {
            components: HashMap::new(),
            resources: HashMap::new(),
            ids: HopSlotMap::with_key(),
            active: true,
        };
        instance.register::<EntityId>();
        instance.add_resource(Commands::default());

        instance
    }

    /// Creates a new `EntityBuilder` for creating and configuring entities within `World`.
    pub fn new_entity(&mut self) -> EntityBuilder {
        let key = self.ids.insert(());

        EntityBuilder::new(key, self)
    }

    /// Removes an entity with a given ID.
    pub fn remove_entity(&mut self, key: EntityId) {
        self.ids.remove(key);
        /*
        TODO: how do we remove components when we delete an entity?
        for storage in self.components.values() {
            storage.borrow_mut().downcast_mut::<ComponentStorage<_>>().unwrap().remove(key);
        }
        */
    }

    /// Registers a component type with the world.
    /// # Warning
    /// Registering a component twice will delete previously stored components!
    pub fn register<T>(&mut self)
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components
            .insert(id, Box::new(ComponentStorage::<T>::new()));
    }

    /// Checks if the world has storage for a specific component type.
    pub fn contains_storage<T>(&self) -> bool
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components.contains_key(&id)
    }

    /// Gets a reference to the storage for a specific component type if it exists.
    fn storage<T>(&self) -> Option<&ComponentStorage<T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components
            .get(&id)?
            .downcast_ref::<ComponentStorage<T>>()
    }

    /// Gets a reference to the storage for a specific component type if it exists.
    fn storage_mut<T>(&mut self) -> Option<&mut ComponentStorage<T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components
            .get_mut(&id)?
            .downcast_mut::<ComponentStorage<T>>()
    }

    /// Checks if an entity has a specific component type.
    ///
    /// This function considers both the existence of the component storage for the type
    /// and the presence of the component associated with the entity ID.
    pub fn contains_component<T>(&self, key: EntityId) -> bool
    where
        T: Component + 'static,
    {
        self.storage::<T>()
            .map(|inner| inner.contains(key))
            .is_some()
            && self.ids.contains_key(key)
    }

    /// Adds a component of a specific type to an entity.
    /// # Panics
    /// Panics if the component `T` has not been registered.
    pub fn add_component<T>(&mut self, key: EntityId, entry: T) -> Option<T>
    where
        T: Component + 'static,
    {
        assert!(
            self.contains_storage::<T>(),
            "Component {} is not registered",
            std::any::type_name::<T>()
        );
        let storage = self.storage_mut::<T>().unwrap();

        storage.insert(key, entry)
    }

    /// Adds a resource to the world.
    pub fn add_resource<T>(&mut self, entry: T) -> Option<T>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<T>();
        let previous_value = self.resources.insert(id, RefCell::new(Box::new(entry)));

        previous_value.map(|inner| *inner.into_inner().downcast::<T>().unwrap())
    }

    /// Removes a component of a specific type from an entity.
    pub fn remove_component<T>(&mut self, key: EntityId) -> Option<T>
    where
        T: Component + 'static,
    {
        let mut storage = self.storage_mut::<T>()?;

        storage.remove(key)
    }

    /// Gets an immutable reference to a resource if it exists.
    pub fn get_resource<T>(&self) -> Option<Ref<'_, T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<T>();

        Some(Ref::map(self.resources.get(&id)?.borrow(), |inner| {
            inner.downcast_ref::<T>().unwrap()
        }))
    }

    /// Gets a mutable reference to a resource if it exists.
    pub fn get_resource_mut<T>(&self) -> Option<RefMut<'_, T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<T>();

        Some(RefMut::map(
            self.resources.get(&id)?.borrow_mut(),
            |inner| inner.downcast_mut::<T>().unwrap(),
        ))
    }

    /// Gets an immutable reference to a component of a specific type associated with an entity if it exists.
    pub fn get_component<T>(&self, key: EntityId) -> Option<Ref<'_, T>>
    where
        T: Component + 'static,
    {
        let storage = self.storage::<T>()?;

        if self.contains_component::<T>(key) {
            storage.get(key)
        } else {
            None
        }
    }

    /// Gets a mutable reference to a component of a specific type associated with an entity if it exists.
    pub fn get_component_mut<T>(&self, key: EntityId) -> Option<RefMut<'_, T>>
    where
        T: Component + 'static,
    {
        let storage = self.storage::<T>()?;

        if self.contains_component::<T>(key) {
            storage.get_mut(key)
        } else {
            None
        }
    }

    /// Gets immutable references to multiple components associated with an entity if it exists.
    pub fn get_components<Q>(&self, key: EntityId) -> Option<Q::Output<'_>>
    where
        Q: Query,
    {
        Q::query(self, key)
    }

    /// Gets mutable references to multiple components associated with an entity if it exists.
    pub fn get_components_mut<Q>(&self, key: EntityId) -> Option<Q::OutputMut<'_>>
    where
        Q: Query,
    {
        Q::query_mut(self, key)
    }

    /// Executes a query on the world to retrieve components associated with entities.
    pub fn query<Q>(&self) -> impl Iterator<Item = <Q>::Output<'_>>
    where
        Q: Query,
    {
        self.ids
            .keys()
            .filter_map(|key| self.get_components::<Q>(key))
    }

    /// Executes a query on the world to retrieve components associated a single entity.
    pub fn query_single<Q>(&self) -> Option<<Q>::Output<'_>>
    where
        Q: Query,
    {
        self.query::<Q>().next()
    }

    /// Executes a query on the world to retrieve components associated with entities.
    pub fn query_mut<Q>(&self) -> impl Iterator<Item = <Q>::OutputMut<'_>>
    where
        Q: Query,
    {
        self.ids
            .keys()
            .filter_map(|key| self.get_components_mut::<Q>(key))
    }

    /// Executes a query on the world to retrieve components associated a single entity.
    pub fn query_single_mut<Q>(&self) -> Option<<Q>::OutputMut<'_>>
    where
        Q: Query,
    {
        self.query_mut::<Q>().next()
    }

    pub fn get_commands(&self) -> RefMut<'_, Commands> {
        self.get_resource_mut::<Commands>().unwrap()
    }

    /// Checks if the `World` is still active.
    pub fn active(&self) -> bool {
        self.active
    }

    /// Shuts down `World`, ending the execution of the app.
    pub fn shutdown(&mut self) {
        self.active = false;
    }
}
