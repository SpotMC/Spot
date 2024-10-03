use crate::block::{
    air::*, birch_leaves::*, birch_log::*, cave_air::*, deepslate::*, dirt::*, grass_block::*,
    oak_leaves::*, oak_log::*, register_block, stone::*,
};

pub(crate) fn register_vanilla() {
    register_block(GRASS_BLOCK, Box::new(GrassBlock::new()));
    register_block(STONE, Box::new(Stone::new()));
    register_block(DIRT, Box::new(Dirt::new()));
    register_block(DEEPSLATE, Box::new(DeepSlate::new()));
    register_block(OAK_LOG, Box::new(OakLog::new()));
    register_block(OAK_LEAVES, Box::new(OakLeaves::new()));
    register_block(BIRCH_LOG, Box::new(BirchLog::new()));
    register_block(BIRCH_LEAVES, Box::new(BirchLeaves::new()));
    register_block(CAVE_AIR, Box::new(CaveAir::new()));
    register_block(AIR, Box::new(Air::new()));
}
