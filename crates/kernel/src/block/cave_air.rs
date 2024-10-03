use crate::block::*;
use crate::{block_def, empty_block_state};
use serde_derive::Deserialize;

pub(crate) const CAVE_AIR: &str = "minecraft:cave_air";
block_def!(
    CaveAir,
    BlockBuilder::new::<CaveAirBlockState>(
        CAVE_AIR,
        BlockSettings::new()
            .strength(0.0)
            .block_type(BlockType::Air)
    )
);
empty_block_state!(CaveAirBlockState, CAVE_AIR);
