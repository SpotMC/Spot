use crate::block::logs::LogProperties;
use crate::block::*;
use crate::block_state;
use serde_derive::Deserialize;

pub const OAK_LOG: &str = "minecraft:oak_log";
pub struct OakLog {
    pub builder: BlockBuilder,
}
impl Block for OakLog {
    fn get_builder(&self) -> &BlockBuilder {
        &self.builder
    }
}

impl OakLog {
    pub(crate) fn new() -> OakLog {
        OakLog {
            builder: BlockBuilder::new::<OakLogBlockState>(
                OAK_LOG,
                BlockSettings::new().strength(2.0),
            ),
        }
    }
}

block_state!(OakLogBlockState, LogProperties, OAK_LOG);
