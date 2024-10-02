use crate::world::dimension::Dimension;
use std::sync::Arc;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BlockUpdateType {
    NeighbourChange,
    PostPlacement,
    Change(u32),
}

pub struct BlockUpdate {
    pub pos: (i32, i32, i32),
    pub dimension: Arc<Dimension>,
    pub state: u32,
    pub(crate) update_type: BlockUpdateType,
}
impl BlockUpdate {
    pub fn new(
        x: i32,
        y: i32,
        z: i32,
        dimension: Arc<Dimension>,
        state: u32,
        update_type: BlockUpdateType,
    ) -> BlockUpdate {
        BlockUpdate {
            pos: (x, y, z),
            dimension,
            state,
            update_type,
        }
    }
}
