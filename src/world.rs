use std::collections::HashMap;
use std::any::TypeId;
use crate::component_array::ComponentArray;
use crate::entity::Entity;

pub struct World {
    generation: usize,
    next_index: usize,
    entities: Vec<Option<Entity>>,
    components: HashMap<TypeId, Box<dyn ComponentArray>>,
}

impl World {
    fn ensure_space(&mut self) {
        const INITIAL_SPACE: usize = 10;
        let entity_count = self.entities.len();
        if entity_count <= self.next_index {
            let new_size = if entity_count == 0 {
                INITIAL_SPACE
            } else {
                entity_count * 2
            };
            self.entities.resize_with_none(new_size);
            for component_array in self.components.values_mut() {
                component_array.resize_with_none(new_size);
            }
        }
    }

    fn update_next_index(&mut self) {
        self.next_index = self.entities.iter()
            .enumerate()
            .skip(self.next_index + 1)
            .find_map(|(index, entity)| if entity.is_none() { Some(index) } else { None })
            .unwrap_or(self.next_index + 1);
    }

    fn check_entity(&self, entity: Entity) -> bool {
        if entity.index < self.entities.len() {
            if let Some(stored) = &self.entities[entity.index] {
                stored.generation == entity.generation
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        self.ensure_space();
        let entity = Entity {
            generation: self.generation,
            index: self.next_index,
        };
        self.entities[self.next_index] = Some(entity);
        self.update_next_index();
        entity
    }

    pub fn update_component<T: 'static>(&mut self, entity: Entity, component: T) {
        if self.check_entity(entity) {
            let type_id = TypeId::of::<T>();
            let components = self.components.entry(type_id).or_insert_with(|| {
                let mut vec = Vec::<Option<T>>::new();
                vec.resize_with_none(self.entities.len());
                Box::new(vec)
            });
            let components = components.as_any_mut().downcast_mut::<Vec<Option<T>>>().unwrap();
            components[entity.index] = Some(component);
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        if self.check_entity(entity) {
            let type_id = TypeId::of::<T>();
            self.components.get(&type_id).and_then(|components| {
                let components = components.as_any().downcast_ref::<Vec<Option<T>>>().unwrap();
                match components.get(entity.index) {
                    Some(Some(component)) => Some(component),
                    _ => None
                }
            })
        } else {
            None
        }
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        if self.check_entity(entity) {
            self.entities[entity.index] = None;
            for components in self.components.values_mut() {
                components.unset(entity.index);
            }
            if entity.index < self.next_index {
                self.next_index = entity.index;
            }
            self.generation += 1;
        }
    }
}