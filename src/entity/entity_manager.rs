use crate::entity::Entity;
use rand::random;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub struct EntityManager<'a> {
    entities: HashMap<i32, Box<dyn Entity + 'a>>,
}

impl<'a> EntityManager<'a> {
    pub fn new() -> EntityManager<'a> {
        EntityManager {
            entities: HashMap::with_capacity(64),
        }
    }
    pub fn spawn<T: Entity + 'a>(&mut self, entity: T) -> i32 {
        let eid: i32 = random();
        self.entities.insert(eid, Box::new(entity));
        eid
    }
    pub fn get_mut(&mut self, eid: i32) -> Option<&mut (dyn Entity + 'a)> {
        match self.entities.get_mut(&eid) {
            Some(e) => Some(e.deref_mut()),
            None => None,
        }
    }
    pub fn get(&self, eid: i32) -> Option<&(dyn Entity + 'a)> {
        match self.entities.get(&eid) {
            Some(e) => Some(e.deref()),
            None => None,
        }
    }
    pub fn remove(&mut self, eid: i32) -> Option<Box<dyn Entity + 'a>> {
        self.entities.remove(&eid)
    }
}

impl<'a> Default for EntityManager<'a> {
    fn default() -> Self {
        EntityManager::new()
    }
}
