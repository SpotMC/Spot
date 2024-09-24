use crate::block::leaves::LeavesProperties;
use crate::block::*;
use crate::{block_def, block_state};
use serde_derive::Deserialize;

pub const OAK_LEAVES: &str = "minecraft:oak_leaves";
block_def! {
    OakLeaves,
    BlockBuilder::new::<OakLeavesBlockState>(OAK_LEAVES, BlockSettings::new().strength(0.2))
}
block_state!(OakLeavesBlockState, LeavesProperties, OAK_LEAVES);
