use crate::entity::entity_manager::EntityManager;
use crate::registry::dimension_type::DIMENSION_TYPES;
use crate::registry::DIMENSION_TYPES_INDEX;
use crate::world::block_update::{BlockUpdate, BlockUpdateType};
use crate::world::dimension::Dimension;
use dashmap::DashSet;
use rayon::prelude::*;
use std::thread;

pub mod block_update;
pub mod chunk;
pub mod dimension;

pub struct World {
    pub dimensions: Vec<Dimension>,
    pub entities: EntityManager<'static>,
    block_updates_queue_1: Vec<BlockUpdate>,
    block_updates_queue_2: Vec<BlockUpdate>,
    use_2: bool,
}
impl World {
    pub fn new() -> World {
        let mut dimensions = Vec::with_capacity(DIMENSION_TYPES_INDEX.len());
        for dim in DIMENSION_TYPES_INDEX.iter() {
            dimensions.push(Dimension::new(
                DIMENSION_TYPES.get(dim).unwrap().value().clone(),
                dim.to_string(),
            ));
        }
        World {
            dimensions,
            entities: EntityManager::default(),
            block_updates_queue_1: Vec::new(),
            block_updates_queue_2: Vec::new(),
            use_2: false,
        }
    }
    #[inline]
    fn get_queue(&mut self) -> &mut Vec<BlockUpdate> {
        if self.use_2 {
            &mut self.block_updates_queue_2
        } else {
            &mut self.block_updates_queue_1
        }
    }
    #[inline]
    fn get_internal_queue(&mut self) -> &mut Vec<BlockUpdate> {
        if self.use_2 {
            &mut self.block_updates_queue_1
        } else {
            &mut self.block_updates_queue_2
        }
    }
    #[inline]
    fn swap_queues(&mut self) {
        self.use_2 = !self.use_2;
    }
    pub fn add_block_update(&mut self, update: BlockUpdate) {
        self.get_queue().push(update);
    }
    pub fn tick(&mut self) {
        self.swap_queues();
        let queue = self.get_internal_queue();
        let blocks_in_use: DashSet<(i32, i32, i32)> = DashSet::new();
        queue.par_iter_mut().for_each(|update| {
            match update.update_type {
                BlockUpdateType::NeighbourChange => {
                    // TODO
                }
                BlockUpdateType::PostPlacement => {
                    // TODO
                }
                BlockUpdateType::Change(new) => {
                    while !blocks_in_use.contains(&update.pos) {
                        thread::yield_now()
                    }
                    update
                        .dimension
                        .set_block(update.pos.0, update.pos.1, update.pos.2, new);
                }
                BlockUpdateType::NeighbourChangeDouble => {
                    // TODO
                }
            }
        });
        self.swap_queues();
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
