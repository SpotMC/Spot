use crate::block::logs::Axis;
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
#[allow(unused)]
pub struct DeepSlateProperties {
    axis: Axis,
}

block_state!(DeepSlateBlockState, DeepSlateProperties, DEEPSLATE);
