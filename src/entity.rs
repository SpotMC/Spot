use crate::world::dimension::Dimension;
use downcast_rs::{impl_downcast, DowncastSync};

pub mod entity_manager;
pub mod player;

pub trait Entity: Send + Sync + DowncastSync {
    fn get_type(&self) -> u32;
    fn get_position(&self) -> (f64, f64, f64);
    fn set_position(&mut self, x: f64, y: f64, z: f64);
    fn get_dimension(&mut self) -> &mut Dimension;
    fn set_dimension(&mut self, dimension: &str);
    fn get_eid(&self) -> i32;
    fn get_velocity(&self) -> (f32, f32, f32);
    fn set_velocity(&mut self, x: f32, y: f32, z: f32);
    fn get_rotation(&self) -> (f32, f32, bool);
    fn set_rotation(&mut self, yaw: f32, pitch: f32, on_ground: bool);
}
impl_downcast!(sync Entity);

pub trait LivingEntity: Entity {
    fn get_health(&self) -> f32;
    fn set_health(&mut self, health: f32);
    fn decrease_health(&mut self, amount: f32);
}
