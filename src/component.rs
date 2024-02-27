#![allow(unused)]
use slotmap::{SecondaryMap, new_key_type};
use std::cell::{Ref, RefCell, RefMut};

pub trait Component {}

new_key_type! {
    pub struct EntityId;
}

// Component store of one type for all entities
#[derive(Default)]
pub struct ComponentStorage<T>
where
    T: Component + 'static,
{
    components: SecondaryMap<EntityId, RefCell<T>>,
}

impl<T> ComponentStorage<T>
where
    T: Component + 'static,
{
    pub fn new() -> Self {
        Self {
            components: SecondaryMap::new(),
        }
    }

    pub fn insert(&mut self, key: EntityId, entry: T) -> Option<RefCell<T>> {
        self.components.insert(key, RefCell::new(entry))
    }

    pub fn remove(&mut self, key: EntityId) -> Option<RefCell<T>> {
        self.components.remove(key)
    }

    pub fn get(&self, key: EntityId) -> Option<Ref<T>> {
        Some(Ref::map(self.components.get(key)?.borrow(), |inner| inner))
    }

    pub fn get_mut(&self, key: EntityId) -> Option<RefMut<T>> {
        Some(RefMut::map(self.components.get(key)?.borrow_mut(), |inner| inner))

    }

    pub fn with_entity<F, R>(&self, key: EntityId, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R
    {
        self.components.get(key).map(|e| f(&*e.borrow()))
    }

    pub fn with_entity_mut<F, R>(&self, key: EntityId, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R
    {
        self.components.get(key).map(|e| f(&mut *e.borrow_mut()))
    }
}
