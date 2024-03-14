#![allow(unused)]
use slotmap::{new_key_type, SecondaryMap};
use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
};

/// A marker trait for components. This trait **must** be implemented for components one wants to add to the `World`.
pub trait Component {}

/// Macro used to automatically generate component structs that implement the `Component` trait.
#[macro_export]
macro_rules! make_component {
    ($(#[$outer:meta])* $vis:vis struct $name:ident $($rest:tt)*) => {
        $(#[$outer])*
        $vis struct $name $($rest)*
        impl $crate::component::Component for $name {}
    };
}

new_key_type! {
    pub struct EntityId;
}

impl Component for EntityId {}

/// A component storage system for holding components of a single type for all entities.
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
    /// Creates a new `ComponentStorage` instance.
    pub fn new() -> Self {
        Self {
            components: SecondaryMap::new(),
        }
    }

    /// Inserts a new component for the given entity ID.
    pub fn insert(&mut self, key: EntityId, entry: T) -> Option<T> {
        self.components
            .insert(key, RefCell::new(entry))
            .map(RefCell::into_inner)
    }

    /// Removes a component from the storage for the given entity ID.
    pub fn remove(&mut self, key: EntityId) -> Option<T> {
        self.components.remove(key).map(RefCell::into_inner)
    }

    /// Checks if a component exists for the given entity ID.
    pub fn contains(&self, key: EntityId) -> bool {
        self.components.contains_key(key)
    }

    /// Gets an immutable reference to the component for the given entity ID. The components are behind a `RefCell`, so it returns `Ref<'_, T>`.
    pub fn get(&self, key: EntityId) -> Option<Ref<'_, T>> {
        self.components.get(key).map(|inner| inner.borrow())
    }

    /// Gets a mutable reference to the component for the given entity ID. The components are behind a `RefCell`, so it returns `RefMut<'_, T>`.
    pub fn get_mut(&self, key: EntityId) -> Option<RefMut<'_, T>> {
        self.components.get(key).map(|inner| inner.borrow_mut())
    }
}
