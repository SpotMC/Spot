use crate::config::VIEW_DISTANCE;
use crate::entity::{Entity, EntityData, LivingEntity};
use crate::registry::protocol_id::get_protocol_id;
use crate::world::chunk::Chunk;
use crate::{impl_entity, impl_living_entity};
use std::sync::Arc;
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
    pub chunks: Vec<Arc<Chunk>>,
    pub tx: UnboundedSender<PlayerUpdate>,
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
            chunks: Vec::with_capacity(512),
            tx,
        }
    }

    pub fn swap(&mut self) {
        let mut del: Vec<usize> = Vec::with_capacity(128);
        let mut i = 0;
        let view_distance = (*VIEW_DISTANCE + 1) as f64 * 16f64;
        while i < self.chunks.len() {
            let chunk = &self.chunks[i];
            let block_x = (chunk.pos >> 32) * 16;
            let block_z = (chunk.pos << 32 >> 32) * 16;
            if (self.entity.pos.0 - block_x as f64).abs() > view_distance
                || (self.entity.pos.2 - block_z as f64).abs() > view_distance
            {
                del.push(i);
            }
            i += 1;
        }
        for i in del.iter().rev() {
            self.chunks.remove(*i);
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
