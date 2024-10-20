use crate::entity::{Entity, EntityData, LivingEntity};
use crate::registry::protocol_id::get_protocol_id;
use crate::{impl_entity, impl_living_entity};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub struct Player {
    pub health: f32,
    pub max_health: u16,
    pub entity: EntityData,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub death_location: Option<(String, i32, i32, i32)>,
    pub teleport_id: Option<i32>,
    pub(crate) tx: UnboundedSender<PlayerUpdate>,
}

impl Player {
    pub fn new(
        entity_id: i32,
        dimension: usize,
        tx: UnboundedSender<PlayerUpdate>,
        pos: (f64, f64, f64),
        uuid: Uuid,
    ) -> Player {
        Player {
            health: 20.0,
            max_health: 20,
            entity: EntityData::with_uuid(entity_id, uuid, dimension, pos),
            game_mode: 0,
            previous_game_mode: -1,
            death_location: None,
            teleport_id: None,
            tx,
        }
    }
}

impl_entity!(Player, entity, "minecraft:player");
impl_living_entity!(Player, health, max_health);

impl PartialEq<Self> for Player {
    fn eq(&self, other: &Self) -> bool {
        self.entity.entity_id == other.entity.entity_id
    }
}

impl Eq for Player {}

pub struct PlayerUpdate {}
