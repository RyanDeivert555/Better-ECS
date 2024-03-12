#![allow(unused)]
use crate::entity_builder::EntityBuilder;
use crate::{component::{Component, ComponentStorage, EntityId}, query::Query};
use slotmap::HopSlotMap;
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

impl World {
    pub fn new() -> Self {
        let mut instance = Self {
            components: HashMap::new(),
            resources: HashMap::new(),
            ids: HopSlotMap::with_key(),
            active: true,
        };
        instance.register::<EntityId>();

        instance
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        let key = self.ids.insert(());

        EntityBuilder::new(key, self)
    }

    pub fn remove_entity(&mut self, key: EntityId) {
        self.ids.remove(key);
        /*
        TODO: how do we remove components when we delete an entity?
        for storage in self.components.values() {
            storage.borrow_mut().downcast_mut::<ComponentStorage<_>>().unwrap().remove(key);
        }
        */
    }

    pub fn register<T>(&mut self)
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components
            .insert(id, Box::new(ComponentStorage::<T>::new()));
    }

    pub fn contains_storage<T>(&self) -> bool
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components.contains_key(&id)
    }

    fn storage<T>(&self) -> Option<&ComponentStorage<T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components
            .get(&id)?
            .downcast_ref::<ComponentStorage<T>>()
    }

    fn storage_mut<T>(&mut self) -> Option<&mut ComponentStorage<T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components
            .get_mut(&id)?
            .downcast_mut::<ComponentStorage<T>>()
    }

    pub fn contains_component<T>(&self, key: EntityId) -> bool
    where
        T: Component + 'static,
    {
        self.storage::<T>()
            .map(|inner| inner.contains(key))
            .is_some()
            && self.ids.contains_key(key)
    }

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

    pub fn add_resource<T>(&mut self, entry: T) -> Option<T>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<T>();
        let previous_value = self.resources.insert(id, RefCell::new(Box::new(entry)));

        previous_value.map(|inner| *inner.into_inner().downcast::<T>().unwrap())
    }

    pub fn remove_component<T>(&mut self, key: EntityId) -> Option<T>
    where
        T: Component + 'static,
    {
        let mut storage = self.storage_mut::<T>()?;

        storage.remove(key)
    }

    pub fn get_resource<T>(&self) -> Option<Ref<'_, T>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<T>();

        Some(Ref::map(self.resources.get(&id)?.borrow(), |inner| {
            inner.downcast_ref::<T>().unwrap()
        }))
    }

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

    pub fn get_components<Q>(&self, key: EntityId) -> Option<Q::Output<'_>>
    where
        Q: Query,
    {
        Q::query(self, key)
    }

    pub fn get_components_mut<Q>(&self, key: EntityId) -> Option<Q::OutputMut<'_>>
    where
        Q: Query,
    {
        Q::query_mut(self, key)
    }

    pub fn query<Q>(&self) -> impl Iterator<Item = <Q>::Output<'_>>
    where
        Q: Query,
    {
        self.ids
            .keys()
            .filter_map(|key| self.get_components::<Q>(key))
    }

    pub fn query_mut<Q>(&self) -> impl Iterator<Item = <Q>::OutputMut<'_>>
    where
        Q: Query,
    {
        self.ids
            .keys()
            .filter_map(|key| self.get_components_mut::<Q>(key))
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn shutdown(&mut self) {
        self.active = false;
    }
}
