use crate::entity::{Entity, LivingEntity};
use crate::registry::protocol_id::get_protocol_id;
use crate::world::dimension::Dimension;
use crate::WORLD;
use tokio::sync::mpsc::UnboundedSender;

pub struct Player {
    pub health: f32,
    pub max_health: u16,
    pub dimension: usize,
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
    pub(crate) tx: UnboundedSender<PlayerUpdate>,
}

impl Player {
    pub fn new(
        entity_id: i32,
        dimension: usize,
        tx: UnboundedSender<PlayerUpdate>,
        pos: (f64, f64, f64),
    ) -> Player {
        Player {
            health: 20.0,
            max_health: 20,
            dimension,
            entity_id,
            game_mode: 0,
            previous_game_mode: -1,
            death_location: None,
            portal_cooldown: 0,
            pos,
            velocity: (0.0, 0.0, 0.0),
            on_ground: false,
            yaw: 0.0,
            pitch: 0.0,
            tx,
        }
    }
}

impl Entity for Player {
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
        unsafe { WORLD.get_mut().dimensions.get_mut(self.dimension).unwrap() }
    }

    fn set_dimension(&mut self, dimension: &str) {
        unsafe {
            self.dimension = WORLD
                .get_mut()
                .dimensions
                .iter()
                .position(|d| d.dimension_name == dimension)
                .unwrap();
        }
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

impl LivingEntity for Player {
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

impl PartialEq<Self> for Player {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for Player {}

pub struct PlayerUpdate {}
unsafe impl Send for PlayerUpdate {}
unsafe impl Sync for PlayerUpdate {}
