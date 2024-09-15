use crate::block::{Block, BlockBuilder, BlockSettings, BlockState, BLOCKS_BY_NAME};
use crate::block_state;
use serde_derive::Deserialize;

pub(crate) const GRASS_BLOCK: &str = "minecraft:grass_block";
pub struct GrassBlock {
    pub builder: BlockBuilder,
}
impl GrassBlock {
    pub(crate) fn new() -> GrassBlock {
        GrassBlock {
            builder: BlockBuilder::new::<GrassBlockState>(
                GRASS_BLOCK,
                BlockSettings::new().strength(0.6),
            ),
        }
    }
}
impl Block for GrassBlock {
    fn get_builder(&self) -> &BlockBuilder {
        &self.builder
    }
}

#[derive(Deserialize)]
pub struct GrassBlockStateProperties {
    pub snowy: bool,
}

block_state! {
    GrassBlockState,
    GrassBlockStateProperties,
    GRASS_BLOCK
}
