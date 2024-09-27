use crate::world::dimension::Dimension;
use crate::WORLD;
use downcast_rs::{impl_downcast, DowncastSync};
use parking_lot::RwLock;
use std::sync::Arc;

pub mod entity_manager;
pub mod player;

pub trait Entity: Send + Sync + DowncastSync {
    fn get_type(&self) -> u32;
    fn get_data_mut(&mut self) -> &mut EntityData;
    fn get_data(&self) -> &EntityData;
    fn get_position(&self) -> (f64, f64, f64) {
        self.get_data().pos
    }

    fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.get_data_mut().pos = (x, y, z);
    }

    fn get_dimension(&mut self) -> Arc<RwLock<Dimension>> {
        unsafe {
            WORLD
                .read()
                .dimensions
                .get(self.get_data().dimension)
                .unwrap()
                .clone()
        }
    }

    fn set_dimension(&mut self, dimension: &str) {
        unsafe {
            self.get_data_mut().dimension = WORLD
                .read()
                .dimensions
                .iter()
                .position(|d| d.read().dimension_name == dimension)
                .unwrap();
        }
    }

    fn get_eid(&self) -> i32 {
        self.get_data().entity_id
    }

    fn get_velocity(&self) -> (f32, f32, f32) {
        self.get_data().velocity
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.get_data_mut().velocity = (x, y, z);
    }

    fn get_rotation(&self) -> (f32, f32, bool) {
        let dat = self.get_data();
        (dat.yaw, dat.pitch, dat.on_ground)
    }

    fn set_rotation(&mut self, yaw: f32, pitch: f32, on_ground: bool) {
        let dat = self.get_data_mut();
        dat.yaw = yaw;
        dat.pitch = pitch;
        dat.on_ground = on_ground;
    }
}
impl_downcast!(sync Entity);

pub trait LivingEntity: Entity {
    fn get_health(&self) -> f32;
    fn set_health(&mut self, health: f32) -> bool;
    fn decrease_health(&mut self, amount: f32) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityData {
    pub dimension: usize,
    pub entity_id: i32,
    pub portal_cooldown: i32,
    pub pos: (f64, f64, f64),
    pub velocity: (f32, f32, f32),
    pub on_ground: bool,
    pub yaw: f32,
    pub pitch: f32,
}
impl EntityData {
    pub fn new(entity_id: i32, dimension: usize, pos: (f64, f64, f64)) -> EntityData {
        EntityData {
            dimension,
            entity_id,
            portal_cooldown: 0,
            pos,
            velocity: (0.0, 0.0, 0.0),
            on_ground: false,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[macro_export]
macro_rules! impl_entity {
    ($class:ty ,$field_name:tt, $id:expr) => {
        impl PartialEq<dyn Entity> for $class {
            fn eq(&self, other: &dyn Entity) -> bool {
                self.get_eid() == other.get_eid()
            }
        }
        impl Entity for $class {
            fn get_type(&self) -> u32 {
                get_protocol_id("minecraft:entity_type", $id).unwrap()
            }

            fn get_data_mut(&mut self) -> &mut EntityData {
                &mut self.$field_name
            }

            fn get_data(&self) -> &EntityData {
                &self.$field_name
            }
        }
    };
}

#[macro_export]
macro_rules! impl_living_entity {
    ($class:ty ,$health:tt, $max_health:tt) => {
        impl LivingEntity for $class {
            fn get_health(&self) -> f32 {
                self.$health
            }

            fn set_health(&mut self, health: f32) -> bool {
                if health > self.$max_health as f32 {
                    self.$health = self.$max_health as f32;
                    return false;
                }
                if health <= 0.0 {
                    self.$health = 0.0;
                    return true;
                }
                self.$health = health;
                false
            }

            fn decrease_health(&mut self, amount: f32) -> bool {
                self.$health -= amount;
                if self.$health <= 0.0 {
                    return true;
                }
                if self.$health > self.$max_health as f32 {
                    self.$health = self.$max_health as f32;
                }
                false
            }
        }
    };
}
