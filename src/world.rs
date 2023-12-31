use std::{borrow::BorrowMut, slice::Iter};

use crate::{component::ComponentVec, entity::Entity};

pub struct World {
    entities_count: usize,
    components: Vec<Box<dyn super::component::ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            components: Vec::new(),
        }
    }

    /// Creates a new entity and returns it's id.
    pub fn new_entity(&mut self) -> Entity {
        let id = self.entities_count;
        for components in self.components.iter_mut() {
            components.push_none();
        }
        self.entities_count += 1;
        Entity::new(id, self)
    }

    fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<&Vec<Option<ComponentType>>> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                return Some(&component_vec);
            };
        }
        None
    }

    pub(crate) fn add_component<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in &mut self.components {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = Some(component);
                return;
            }
        }

        let mut new_components_vec = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_components_vec.push(None);
        }
        new_components_vec[entity] = Some(component);
        self.components.push(Box::new(new_components_vec))
    }

    pub fn query<Query>(&self) -> Option<Box<dyn Iterator<Item = Query>>> {
        if let Some(component_vec) = self.borrow_component_vec::<Query>() {
            for component in  component_vec.iter().filter_map(|q| q.as_ref());
        };
        None
    }
}
