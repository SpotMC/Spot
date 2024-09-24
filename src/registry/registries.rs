use crate::block::birch_leaves::{BirchLeaves, BIRCH_LEAVES};
use crate::block::birch_log::{BirchLog, BIRCH_LOG};
use crate::block::deepslate::{DeepSlate, DEEPSLATE};
use crate::block::dirt::{Dirt, DIRT};
use crate::block::grass_block::{GrassBlock, GRASS_BLOCK};
use crate::block::oak_leaves::{OakLeaves, OAK_LEAVES};
use crate::block::oak_log::{OakLog, OAK_LOG};
use crate::block::register_block;
use crate::block::stone::{Stone, STONE};

pub(crate) fn register_vanilla() {
    register_block(GRASS_BLOCK, Box::new(GrassBlock::new()));
    register_block(STONE, Box::new(Stone::new()));
    register_block(DIRT, Box::new(Dirt::new()));
    register_block(DEEPSLATE, Box::new(DeepSlate::new()));
    register_block(OAK_LOG, Box::new(OakLog::new()));
    register_block(OAK_LEAVES, Box::new(OakLeaves::new()));
    register_block(BIRCH_LOG, Box::new(BirchLog::new()));
    register_block(BIRCH_LEAVES, Box::new(BirchLeaves::new()));
}
