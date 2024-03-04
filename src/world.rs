#![allow(unused)]
use crate::component::*;
use crate::entity::EntityBuilder;
use slotmap::SlotMap;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

type ComponentMap = HashMap<TypeId, RefCell<Box<dyn Any>>>;

pub struct World {
    // has ComponentStorage<T>
    components: ComponentMap,
    ids: SlotMap<EntityId, ()>,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            ids: SlotMap::with_key(),
        }
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
            .insert(id, RefCell::new(Box::new(ComponentStorage::<T>::new())));
    }

    pub fn contains_storage<T>(&self) -> bool
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components.contains_key(&id)
    }

    pub fn storage<T>(&self) -> Option<Ref<'_, ComponentStorage<T>>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        Some(Ref::map(self.components.get(&id)?.borrow(), |inner| {
            inner.downcast_ref::<ComponentStorage<T>>().unwrap()
        }))
    }

    pub fn storage_mut<T>(&self) -> Option<RefMut<'_, ComponentStorage<T>>>
    where
        T: Component + 'static,
    {
        let id = TypeId::of::<ComponentStorage<T>>();

        Some(RefMut::map(
            self.components.get(&id)?.borrow_mut(),
            |inner| inner.downcast_mut::<ComponentStorage<T>>().unwrap(),
        ))
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

    pub fn add_component<T>(&mut self, key: EntityId, entry: T)
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>(), "Component is not registered");
        let mut storage = self.storage_mut::<T>().unwrap();
        storage.insert(key, entry);
    }

    pub fn remove_component<T>(&mut self, key: EntityId)
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>());
        {
            let mut storage = self.storage_mut::<T>().unwrap();
            storage.remove(key);
        }
        let id = TypeId::of::<ComponentStorage<T>>();

        self.components.remove(&id);
    }

    pub fn get_component<T>(&self, key: EntityId) -> Option<Ref<T>>
    where
        T: Component + 'static,
    {
        if self.contains_component::<T>(key) {
            Ref::filter_map(self.storage::<T>()?, |inner| inner.get(key)).ok()
        } else {
            None
        }
    }

    pub fn get_component_mut<T>(&self, key: EntityId) -> Option<RefMut<T>>
    where
        T: Component + 'static,
    {
        if self.contains_component::<T>(key) {
            RefMut::filter_map(self.storage_mut::<T>()?, |inner| inner.get_mut(key)).ok()
        } else {
            None
        }
    }
}
