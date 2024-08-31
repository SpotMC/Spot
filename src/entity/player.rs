use crate::entity::{Entity, LivingEntity};
use crate::network::connection::Connection;
use crate::registry::protocol_id::get_protocol_id;
use crate::world::dimension::Dimension;

pub struct Player<'a> {
    pub(crate) connection: Connection<'a>,
    pub health: f32,
    pub max_health: u16,
    pub dimension: &'static mut Dimension,
    pub(crate) entity_id: i32,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub death_location: Option<(String, i32, i32, i32)>,
    pub portal_cooldown: i32,
    pub pos: (f64, f64, f64),
    pub velocity: (f32, f32, f32),
    pub on_ground: bool,
    pub yaw: f32,
    pub pitch: f32,
}

impl Entity for Player<'_> {
    fn get_type(&self) -> u32 {
        get_protocol_id("minecraft:entity_type", "minecraft:player").unwrap()
    }

    fn get_position(&self) -> (f64, f64, f64) {
        self.pos
    }

    fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.pos = (x, y, z);
    }

    fn get_dimension(&mut self) -> &mut Dimension {
        self.dimension
    }

    fn set_dimension(&mut self, dimension: &'static mut Dimension) {
        self.dimension = dimension;
    }

    fn get_eid(&self) -> i32 {
        self.entity_id
    }

    fn get_velocity(&self) -> (f32, f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity = (x, y, z);
    }

    fn get_rotation(&self) -> (f32, f32, bool) {
        (self.yaw, self.pitch, self.on_ground)
    }

    fn set_rotation(&mut self, yaw: f32, pitch: f32, on_ground: bool) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.on_ground = on_ground;
    }
}

impl LivingEntity for Player<'_> {
    fn get_health(&self) -> f32 {
        self.health
    }

    fn set_health(&mut self, health: f32) {
        self.health = health;
    }

    fn decrease_health(&mut self, amount: f32) {
        self.health -= amount;
    }
}
