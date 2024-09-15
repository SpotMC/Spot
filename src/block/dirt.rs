use crate::block::*;
use crate::empty_block_state;
pub(crate) const DIRT: &str = "minecraft:dirt";
pub struct Dirt {
    pub builder: BlockBuilder,
}

impl Dirt {
    pub(crate) fn new() -> Dirt {
        Dirt {
            builder: BlockBuilder::new::<DirtBlockState>(DIRT, BlockSettings::new().strength(0.6)),
        }
    }
}

impl Block for Dirt {
    fn get_builder(&self) -> &BlockBuilder {
        &self.builder
    }
}

empty_block_state!(DirtBlockState, DIRT);
