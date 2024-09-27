use crate::block::leaves::LeavesProperties;
use crate::block::*;
use crate::{block_def, block_state};
use serde_derive::Deserialize;

pub const BIRCH_LEAVES: &str = "minecraft:birch_leaves";
block_def! {
    BirchLeaves,
    BlockBuilder::new::<BirchLeavesBlockState>(BIRCH_LEAVES, BlockSettings::new().strength(0.2))
}
block_state!(BirchLeavesBlockState, LeavesProperties, BIRCH_LEAVES);
