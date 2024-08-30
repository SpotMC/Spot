use crate::world::block_update::BlockUpdateType;
use crate::world::dimension::Dimension;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub trait Block: Send + Sync {
    fn when_block_update(
        &self,
        update_type: BlockUpdateType,
        pos: (i32, i32, i32),
        dimension: &mut Dimension,
        state: u32,
    );
    fn get_block_id(&self) -> u32;
    fn get_default_block_state(&self) -> u32;
    fn get_block_states(&self) -> HashMap<u32, Box<(dyn BlockState)>>;
}
pub trait BlockState: Send + Sync {
    fn get_block_id(&self) -> u32;
    fn get_block_state(&self) -> u32;
}

lazy_static! {
    pub static ref BLOCKS_BY_ID: DashMap<String, Box<dyn Block>> = DashMap::new();
    pub static ref BLOCKS_BY_NAME: DashMap<u32, String> = DashMap::new();
    pub static ref BLOCK_STATES_BY_ID: DashMap<u32, Box<(dyn BlockState)>> = DashMap::new();
}

fn register_block(id: String, block: Box<dyn Block + 'static>) {
    block.get_block_states().into_iter().for_each(|(k, v)| {
        BLOCK_STATES_BY_ID.insert(k, v);
    });
    BLOCKS_BY_NAME.insert(block.get_block_id(), id.clone());
    BLOCKS_BY_ID.insert(id.clone(), block);
}
