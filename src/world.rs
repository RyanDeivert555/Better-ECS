#![allow(unused)]
use crate::component::*;
use crate::entity::EntityBuilder;
use anymap::AnyMap;
use slotmap::SlotMap;

pub struct World {
    // has ComponentStorage<T>
    components: AnyMap,
    ids: SlotMap<EntityId, ()>,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: AnyMap::new(),
            ids: SlotMap::with_key(),
        }
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        let key = self.ids.insert(());

        EntityBuilder::new(key, self)
    }

    pub fn remove_entity(&mut self, key: EntityId) {
        self.ids.remove(key);
    }

    pub fn register<T>(&mut self)
    where
        T: Component + 'static,
    {
        self.components.insert(ComponentStorage::<T>::new());
    }

    pub fn contains_storage<T>(&self) -> bool
    where
        T: Component + 'static,
    {
        self.components.contains::<ComponentStorage<T>>()
    }

    pub fn storage<T>(&self) -> Option<&ComponentStorage<T>>
    where
        T: Component + 'static,
    {
        self.components.get::<ComponentStorage<T>>()
    }

    pub fn storage_mut<T>(&mut self) -> Option<&mut ComponentStorage<T>>
    where
        T: Component + 'static,
    {
        self.components.get_mut::<ComponentStorage<T>>()
    }

    pub fn add_component<T>(&mut self, key: EntityId, entry: T)
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>(), "Component is not registered");
        let storage = self.storage_mut::<T>().unwrap();
        storage.insert(key, entry);
    }

    pub fn remove_component<T>(&mut self, key: EntityId)
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>());
        let storage = self.storage_mut::<T>().unwrap();
        storage.remove(key);
        self.components.remove::<ComponentStorage<T>>();
    }

    pub fn get_component<T>(&self, key: EntityId) -> Option<&T>
    where
        T: Component + 'static,
    {
        let storage = self.storage::<T>()?;

        storage.get(key)
    }

    pub fn get_component_mut<T>(&mut self, key: EntityId) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        let storage = self.storage_mut::<T>()?;

        storage.get_mut(key)
    }

    pub fn query<T, U>(&self) -> impl Iterator<Item = (&T, &U)>
    where
        T: Component + 'static,
        U: Component + 'static,
    {
        self.ids.keys().filter_map(|key| {
            let t = self.get_component::<T>(key);
            let u = self.get_component::<U>(key);

            Some((t?, u?))
        })
    }

    pub fn query_mut<T, U>(&mut self) -> impl Iterator<Item = (&mut T, &mut U)>
    where
        T: Component + 'static,
        U: Component + 'static
    {
        self.ids.keys().filter_map(|key| {
            let t = self.get_component_mut::<T>(key);
            let u = self.get_component_mut::<U>(key);

            Some((t?, u?))
        })
    }
}
