use crate::block::logs::LogProperties;
use crate::block::*;
use crate::block_state;
use serde_derive::Deserialize;

pub const BIRCH_LOG: &str = "minecraft:birch_log";
pub struct BirchLog {
    pub builder: BlockBuilder,
}
impl Block for BirchLog {
    fn get_builder(&self) -> &BlockBuilder {
        &self.builder
    }
}

impl BirchLog {
    pub(crate) fn new() -> BirchLog {
        BirchLog {
            builder: BlockBuilder::new::<BirchLogBlockState>(
                BIRCH_LOG,
                BlockSettings::new().strength(2.0),
            ),
        }
    }
}

block_state!(BirchLogBlockState, LogProperties, BIRCH_LOG);
