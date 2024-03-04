#![allow(unused)]
use slotmap::{new_key_type, SecondaryMap};

pub trait Component {}

#[macro_export]
macro_rules! make_component {
    { $(#[$outer:meta])* $vis:vis struct $name:ident $($rest:tt)* } => {
        $(#[$outer])*
        $vis struct $name $($rest)*
        impl $crate::component::Component for $name {}
    };
}

new_key_type! {
    pub struct EntityId;
}

// Component store of one type for all entities
#[derive(Default)]
pub struct ComponentStorage<T>
where
    T: Component + 'static,
{
    components: SecondaryMap<EntityId, T>,
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
        self.components.insert(key, entry)
    }

    pub fn remove(&mut self, key: EntityId) -> Option<T> {
        self.components.remove(key)
    }

    pub fn contains(&self, key: EntityId) -> bool {
        self.components.contains_key(key)
    }

    pub fn get(&self, key: EntityId) -> Option<&T> {
        self.components.get(key)
    }

    pub fn get_mut(&mut self, key: EntityId) -> Option<&mut T> {
        self.components.get_mut(key)
    }
}
