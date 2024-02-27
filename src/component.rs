use slotmap::*;

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

    pub fn get(&self, key: EntityId) -> Option<&T> {
        self.components.get(key)
    }

    pub fn get_mut(&mut self, key: EntityId) -> Option<&mut T> {
        self.components.get_mut(key)
    }

    pub fn query(&self) -> impl Iterator<Item = &T> {
        self.components.values()
    }

    pub fn query_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.components.values_mut()
    }
}
