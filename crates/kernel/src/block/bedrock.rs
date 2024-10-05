use crate::block::*;
use crate::{block_def, empty_block_state};
use serde_derive::Deserialize;

pub(crate) const BEDROCK: &str = "minecraft:bedrock";
block_def!(
    Bedrock,
    BlockBuilder::new::<BedrockBlockState>(
        BEDROCK,
        BlockSettings::new()
            .hardness(-1.0)
            .resistance(3600000.0)
            .piston_behavior(PistonBehavior::NONE)
            .block_type(BlockType::Solid)
    )
);
empty_block_state!(BedrockBlockState, BEDROCK);
