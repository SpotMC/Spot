use crate::entity::Entity;
use std::collections::HashMap;

pub struct EntityManager {
    entities: HashMap<i32, Entity>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: HashMap::with_capacity(128),
        }
    }
    pub fn spawn(&mut self, entity: Entity, eid: i32) {
        self.entities.insert(eid, entity);
    }
    pub fn get_mut(&mut self, eid: i32) -> Option<&mut Entity> {
        match self.entities.get_mut(&eid) {
            Some(e) => Some(e),
            None => None,
        }
    }
    pub fn get(&self, eid: i32) -> Option<&Entity> {
        match self.entities.get(&eid) {
            Some(e) => Some(e),
            None => None,
        }
    }
    pub fn remove(&mut self, eid: i32) -> Option<Entity> {
        self.entities.remove(&eid)
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        EntityManager::new()
    }
}
