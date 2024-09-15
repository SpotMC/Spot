use crate::block::dirt::{Dirt, DIRT};
use crate::block::grass_block::{GrassBlock, GRASS_BLOCK};
use crate::block::register_block;
use crate::block::stone::{Stone, STONE};

pub(crate) fn register_vanilla() {
    register_block(GRASS_BLOCK, Box::new(GrassBlock::new()));
    register_block(STONE, Box::new(Stone::new()));
    register_block(DIRT, Box::new(Dirt::new()));
}
