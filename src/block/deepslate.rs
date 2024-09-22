use crate::block::*;
use crate::block_state;
use serde_derive::Deserialize;

pub const DEEPSLATE: &str = "minecraft:deepslate";
pub struct DeepSlate {
    pub builder: BlockBuilder,
}
impl Block for DeepSlate {
    fn get_builder(&self) -> &BlockBuilder {
        &self.builder
    }
}
impl DeepSlate {
    pub(crate) fn new() -> DeepSlate {
        DeepSlate {
            builder: BlockBuilder::new::<DeepSlateBlockState>(
                DEEPSLATE,
                BlockSettings::new().hardness(3.0).resistance(6.0),
            ),
        }
    }
}

#[derive(Deserialize)]
pub struct DeepSlateProperties {
    axis: String,
}

block_state!(DeepSlateBlockState, DeepSlateProperties, DEEPSLATE);
