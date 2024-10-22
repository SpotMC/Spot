use crate::block::BLOCKS_BY_ID;
use crate::entity::entity_manager::EntityManager;
use crate::registry::dimension_type::DIMENSION_TYPES;
use crate::registry::DIMENSION_TYPES_INDEX;
use crate::util::raw::Raw;
use crate::world::block_update::BlockUpdateType::{NeighbourChange, PostPlacement};
use crate::world::block_update::{BlockUpdate, BlockUpdateType};
use crate::world::dimension::Dimension;
use dashmap::DashSet;
use parking_lot::Mutex;
use rayon::prelude::*;
use spotlight::event::{ActionResult, EventCallback};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub mod block_update;
pub mod chunk;
pub mod dimension;
pub mod gen;
mod height_map;

static WORLD_TICK_CALLBACK: EventCallback<Raw<World>> = EventCallback::new();

pub struct World {
    default_dimension: usize,
    pub dimensions: Vec<Arc<Dimension>>,
    pub entities: EntityManager,
    block_update_queue_0: Mutex<Vec<BlockUpdate>>,
    block_update_queue_1: Mutex<Vec<BlockUpdate>>,
    use_2: AtomicBool,
}
impl World {
    pub(crate) fn new() -> World {
        let mut dimensions = Vec::with_capacity(DIMENSION_TYPES_INDEX.len());
        let mut idx = 0;
        while idx < DIMENSION_TYPES_INDEX.len() {
            let dim = DIMENSION_TYPES_INDEX.get(idx).unwrap();
            dimensions.push(Arc::new(Dimension::new(
                DIMENSION_TYPES.get(dim).unwrap().value().clone(),
                dim.to_string(),
                idx as u32,
            )));
            idx += 1;
        }
        World {
            default_dimension: dimensions
                .iter()
                .position(|it| it.dimension_name == "minecraft:overworld")
                .unwrap(),
            dimensions,
            block_update_queue_0: Mutex::new(Vec::new()),
            block_update_queue_1: Mutex::new(Vec::new()),
            entities: EntityManager::default(),
            use_2: AtomicBool::new(false),
        }
    }
    #[inline]
    fn get_queue(&self) -> &Mutex<Vec<BlockUpdate>> {
        if self.use_2.load(Ordering::SeqCst) {
            &self.block_update_queue_0
        } else {
            &self.block_update_queue_1
        }
    }
    #[inline]
    fn get_internal_queue(&self) -> &Mutex<Vec<BlockUpdate>> {
        if self.use_2.load(Ordering::SeqCst) {
            &self.block_update_queue_1
        } else {
            &self.block_update_queue_0
        }
    }
    #[inline]
    fn swap_queues(&self) {
        self.use_2
            .store(!self.use_2.load(Ordering::SeqCst), Ordering::SeqCst);
    }
    /// Adds a new block update to the block update queue.
    ///
    /// ## Parameters
    /// - `update`: The block update information, of type `BlockUpdate`.
    pub fn add_block_update(&mut self, update: BlockUpdate) {
        self.get_queue().lock().push(update);
    }
    /// Executes a single tick operation to process all block update events before executing.
    pub fn tick(&self) {
        if let ActionResult::Fail = WORLD_TICK_CALLBACK.interact(Raw::from(self)) {
            return;
        }
        self.swap_queues();
        let mut queue = self.get_internal_queue().lock();
        let new: Mutex<Vec<BlockUpdate>> = Mutex::new(Vec::new());
        loop {
            let blocks_in_use: DashSet<(i32, i32, i32)> = DashSet::new();
            queue
                .par_iter_mut()
                .for_each(|update| match update.update_type {
                    NeighbourChange => {
                        let positions = [
                            (update.pos.0, update.pos.1, update.pos.2 - 1),
                            (update.pos.0, update.pos.1, update.pos.2 + 1),
                            (update.pos.0 + 1, update.pos.1, update.pos.2),
                            (update.pos.0 - 1, update.pos.1, update.pos.2),
                            (update.pos.0, update.pos.1 - 1, update.pos.2),
                            (update.pos.0, update.pos.1 + 1, update.pos.2),
                        ];
                        let mut new = new.lock();
                        for pos in positions.iter() {
                            while blocks_in_use.contains(pos) {
                                rayon::yield_now();
                            }
                            blocks_in_use.insert(*pos);
                            new.append(
                                &mut BLOCKS_BY_ID
                                    .get(&update.state)
                                    .unwrap()
                                    .value()
                                    .when_block_update(
                                        NeighbourChange,
                                        update.pos,
                                        update.dimension.clone(),
                                        update.state,
                                    ),
                            );
                            blocks_in_use.remove(pos);
                        }
                    }
                    PostPlacement => {
                        let positions = [
                            (update.pos.0, update.pos.1, update.pos.2 - 1),
                            (update.pos.0, update.pos.1, update.pos.2 + 1),
                            (update.pos.0, update.pos.1 - 1, update.pos.2),
                            (update.pos.0, update.pos.1 + 1, update.pos.2),
                            (update.pos.0 + 1, update.pos.1, update.pos.2),
                            (update.pos.0 - 1, update.pos.1, update.pos.2),
                        ];
                        let mut new = new.lock();
                        for pos in positions.iter() {
                            while blocks_in_use.contains(pos) {
                                rayon::yield_now();
                            }
                            blocks_in_use.insert(*pos);
                            new.append(
                                &mut BLOCKS_BY_ID
                                    .get(&update.state)
                                    .unwrap()
                                    .value()
                                    .when_block_update(
                                        PostPlacement,
                                        update.pos,
                                        update.dimension.clone(),
                                        update.state,
                                    ),
                            );
                            blocks_in_use.remove(pos);
                        }
                    }
                    BlockUpdateType::Change(new_state) => {
                        while blocks_in_use.contains(&update.pos) {
                            rayon::yield_now();
                        }
                        blocks_in_use.insert(update.pos);
                        update.dimension.set_block(
                            update.pos.0,
                            update.pos.1,
                            update.pos.2,
                            new_state,
                        );
                        let mut new = new.lock();
                        new.append(
                            &mut BLOCKS_BY_ID
                                .get(&new_state)
                                .unwrap()
                                .value()
                                .when_block_update(
                                    update.update_type,
                                    update.pos,
                                    update.dimension.clone(),
                                    update.state,
                                ),
                        );
                        blocks_in_use.remove(&update.pos);
                        new.push(BlockUpdate::new(
                            update.pos.0,
                            update.pos.1,
                            update.pos.2,
                            update.dimension.clone(),
                            new_state,
                            PostPlacement,
                        ))
                    }
                });
            let mut new = new.lock();
            if new.is_empty() {
                break;
            } else {
                queue.clear();
                queue.append(&mut new);
                new.clear();
            }
        }
        self.swap_queues();
    }

    /// Find a dimension by name
    ///
    /// # Parameters
    /// - `name`: &str, the name of the dimension
    ///
    /// # Returns
    /// - `Option<Arc<Dimension>>`: Returns `Some(Arc<Dimension>)` if found, otherwise returns `None`
    ///
    /// # Description
    /// This method iterates through the internal dimensions collection, finds the first dimension that matches the given name, and returns its clone. If no matching dimension is found, it returns `None`.
    pub fn find_dimension(&self, name: &str) -> Option<Arc<Dimension>> {
        Some(
            self.dimensions
                .iter()
                .find(|&dim| dim.dimension_name == name)?
                .clone(),
        )
    }

    pub fn get_world_spawn_point(&self) -> (usize, i32, i32, i32) {
        // TODO
        (self.default_dimension, 0, 0, 0)
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
