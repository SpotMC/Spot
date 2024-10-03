use crate::entity::Entity;
use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct EntityManager {
    entities: DashMap<i32, Arc<Mutex<dyn Entity>>>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: DashMap::with_capacity(128),
        }
    }
    pub fn spawn_into<T: Entity>(&self, entity: T, eid: i32) {
        self.entities.insert(eid, Arc::new(Mutex::new(entity)));
    }
    pub fn spawn(&self, entity: &Arc<Mutex<dyn Entity>>) {
        self.entities
            .insert(entity.lock().get_eid(), entity.clone());
    }
    pub fn get_mut(&self, eid: i32) -> Option<Arc<Mutex<dyn Entity>>> {
        match self.entities.get_mut(&eid) {
            Some(e) => Some(e.value().clone()),
            None => None,
        }
    }
    pub fn remove(&self, eid: i32) -> Option<(i32, Arc<Mutex<dyn Entity>>)> {
        self.entities.remove(&eid)
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        EntityManager::new()
    }
}
