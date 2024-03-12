#![allow(unused)]
use slotmap::{new_key_type, SecondaryMap};
use std::cell::{Ref, RefCell, RefMut};

pub trait Component {}

#[macro_export]
macro_rules! make_component {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident $($rest:tt)* ) => {
        $(#[$outer])*
        $vis struct $name $($rest)*
        impl $crate::component::Component for $name {}
    };
}

new_key_type! {
    pub struct EntityId;
}

impl Component for EntityId {}

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

    pub fn insert(&mut self, key: EntityId, entry: T) -> Option<T> {
        self.components
            .insert(key, RefCell::new(entry))
            .map(RefCell::into_inner)
    }

    pub fn remove(&mut self, key: EntityId) -> Option<T> {
        self.components.remove(key).map(RefCell::into_inner)
    }

    pub fn contains(&self, key: EntityId) -> bool {
        self.components.contains_key(key)
    }

    pub fn get(&self, key: EntityId) -> Option<Ref<'_, T>> {
        self.components.get(key).map(|inner| inner.borrow())
    }

    pub fn get_mut(&self, key: EntityId) -> Option<RefMut<'_, T>> {
        self.components.get(key).map(|inner| inner.borrow_mut())
    }
}
