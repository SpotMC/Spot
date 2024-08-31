use crate::world::block_update::BlockUpdateType;
use crate::world::dimension::Dimension;
use dashmap::DashMap;
use once_cell::sync::Lazy;
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
pub(crate) static BLOCKS_BY_ID: Lazy<DashMap<String, Box<dyn Block>>> = Lazy::new(DashMap::new);
pub(crate) static BLOCKS_BY_NAME: Lazy<DashMap<u32, String>> = Lazy::new(DashMap::new);
pub(crate) static BLOCK_STATES_BY_ID: Lazy<DashMap<u32, Box<(dyn BlockState)>>> =
    Lazy::new(DashMap::new);

fn register_block(id: String, block: Box<dyn Block + 'static>) {
    block.get_block_states().into_iter().for_each(|(k, v)| {
        BLOCK_STATES_BY_ID.insert(k, v);
    });
    BLOCKS_BY_NAME.insert(block.get_block_id(), id.clone());
    BLOCKS_BY_ID.insert(id.clone(), block);
}
