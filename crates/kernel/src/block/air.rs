use crate::block::*;
use crate::{block_def, empty_block_state};
use serde_derive::Deserialize;

pub(crate) const AIR: &str = "minecraft:air";
block_def!(
    Air,
    BlockBuilder::new::<AirBlockState>(
        AIR,
        BlockSettings::new()
            .strength(0.0)
            .block_type(BlockType::Air)
    )
);
empty_block_state!(AirBlockState, AIR);
