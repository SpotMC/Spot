use crate::block::*;
use crate::empty_block_state;
use serde_derive::Deserialize;
pub(crate) const STONE: &str = "minecraft:stone";
pub struct Stone {
    pub builder: BlockBuilder,
}

impl Stone {
    pub(crate) fn new() -> Stone {
        Stone {
            builder: BlockBuilder::new::<StoneBlockState>(
                STONE,
                BlockSettings::new().hardness(1.5).resistance(6.0),
            ),
        }
    }
}

impl Block for Stone {
    fn get_builder(&self) -> &BlockBuilder {
        &self.builder
    }
}

empty_block_state!(StoneBlockState, STONE);
