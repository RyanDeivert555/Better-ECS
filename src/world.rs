use crate::component::*;
use slotmap::SlotMap;
use anymap::AnyMap;
use crate::entity::EntityBuilder;

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
        T: Component + 'static
    {
        self.components.get::<ComponentStorage<T>>()
    }

    pub fn storage_mut<T>(&mut self) -> Option<&mut ComponentStorage<T>>
    where
        T: Component + 'static
    {
        self.components.get_mut::<ComponentStorage<T>>()
    }

    pub fn add_component<T>(&mut self, key: EntityId, entry: T)
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>());
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
    }

    pub fn get_component<T>(&self, key: EntityId) -> Option<&T>
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>());
        let storage = self.storage::<T>().unwrap();

        storage.get(key)
    }

    pub fn get_component_mut<T>(&mut self, key: EntityId) -> Option<&mut T>
    where
        T: Component + 'static,
    {
        assert!(self.contains_storage::<T>());
        let storage = self.storage_mut::<T>().unwrap();

        storage.get_mut(key)
    }
}