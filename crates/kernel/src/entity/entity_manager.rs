use crate::entity::Entity;
use crate::registry::protocol_id::get_protocol_id;
use crate::world::dimension::Dimension;
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

    pub fn lookup(&self) -> EntityLookup {
        EntityLookup {
            entities: self.entities.clone(),
        }
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        EntityManager::new()
    }
}

#[derive(Clone)]
pub struct EntityLookup {
    entities: DashMap<i32, Arc<Mutex<dyn Entity>>>,
}

impl EntityLookup {
    pub fn entity_type(mut self, id: &str) -> Option<Self> {
        let id = get_protocol_id("minecraft:entity_type", id)?;
        let new: DashMap<i32, Arc<Mutex<dyn Entity>>> = DashMap::default();
        self.entities.iter().for_each(|e| {
            if e.value().lock().get_type() == id {
                new.insert(*e.key(), e.value().clone());
            }
        });
        self.entities = new;
        Some(self)
    }

    pub fn dimension(mut self, dimension: Arc<Dimension>) -> Option<Self> {
        let dimension = dimension.dim_idx;
        let new: DashMap<i32, Arc<Mutex<dyn Entity>>> = DashMap::default();
        self.entities.iter().for_each(|e| {
            if e.value().lock().get_dimension().dim_idx == dimension {
                new.insert(*e.key(), e.value().clone());
            }
        });
        self.entities = new;
        Some(self)
    }

    pub fn distance(
        mut self,
        x: f64,
        y: f64,
        z: f64,
        radius: f64,
        dimension: Arc<Dimension>,
    ) -> Option<Self> {
        self = self.manhattan_distance(x, y, z, radius, dimension)?;
        let new: DashMap<i32, Arc<Mutex<dyn Entity>>> = DashMap::default();
        let r = radius.powi(2);
        self.entities.iter().for_each(|e| {
            let entity = e.value().lock();
            let (x1, y1, z1) = entity.get_position();
            let (x2, y2, z2) = (x, y, z);
            let distance = (x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2);
            if distance <= r {
                new.insert(*e.key(), e.value().clone());
            }
        });
        self.entities = new;
        Some(self)
    }

    pub fn manhattan_distance(
        mut self,
        x: f64,
        y: f64,
        z: f64,
        distance: f64,
        dimension: Arc<Dimension>,
    ) -> Option<Self> {
        self = self.dimension(dimension)?;
        let new: DashMap<i32, Arc<Mutex<dyn Entity>>> = DashMap::default();
        self.entities.iter().for_each(|e| {
            let entity = e.value().lock();
            let (x1, y1, z1) = entity.get_position();
            let (x2, y2, z2) = (x, y, z);
            let d = (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs();
            if d <= distance {
                new.insert(*e.key(), e.value().clone());
            }
        });
        self.entities = new;
        Some(self)
    }

    pub fn filter<F: FnMut(&dyn Entity) -> bool>(mut self, mut filter: F) -> Self {
        let new: DashMap<i32, Arc<Mutex<dyn Entity>>> = DashMap::default();
        self.entities.iter().for_each(|e| {
            if filter(&*e.value().lock()) {
                new.insert(*e.key(), e.value().clone());
            }
        });
        self.entities = new;
        self
    }
}
