pub mod birch_leaves;
pub mod birch_log;
pub mod deepslate;
pub mod dirt;
pub mod grass_block;
pub mod leaves;
pub mod logs;
pub mod oak_leaves;
pub mod oak_log;
pub mod stone;

use crate::registry::protocol_id::{get_block_states, get_protocol_id};
use crate::world::block_update::{BlockUpdate, BlockUpdateType};
use crate::world::dimension::Dimension;
use dashmap::DashMap;
use downcast_rs::{impl_downcast, DowncastSync};
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub trait Block: Send + Sync + DowncastSync {
    fn when_block_update(
        &self,
        _update_type: BlockUpdateType,
        _pos: (i32, i32, i32),
        _dimension: Arc<Dimension>,
        _state: u32,
    ) -> Vec<BlockUpdate> {
        Vec::with_capacity(0)
    }
    fn get_builder(&self) -> &BlockBuilder;
    fn get_block_id(&self) -> u32 {
        self.get_builder().protocol_id
    }
    fn get_default_block_state(&self) -> u32 {
        self.get_builder().default_state
    }
    fn get_block_states(&self) -> &HashMap<u32, Arc<(dyn BlockState)>> {
        &self.get_builder().block_states
    }
    fn get_block_settings(&self) -> &BlockSettings {
        &self.get_builder().block_settings
    }
}
impl_downcast!(sync Block);
pub trait BlockState: Send + Sync + DowncastSync {
    fn get_block_id(&self) -> u32;
    fn get_block_state(&self) -> u32;
    fn is_default(&self) -> bool;
}
impl_downcast!(sync BlockState);

pub(crate) static BLOCKS_BY_ID: Lazy<DashMap<u32, Box<dyn Block>>> = Lazy::new(DashMap::new);
pub(crate) static BLOCKS_BY_NAME: Lazy<DashMap<String, u32>> = Lazy::new(DashMap::new);
pub(crate) static BLOCK_STATES_BY_ID: Lazy<DashMap<u32, Arc<(dyn BlockState)>>> =
    Lazy::new(DashMap::new);
pub(crate) static BLOCK_ITEM_BY_ID: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

pub fn register_block(id: &str, block: Box<dyn Block + 'static>) {
    block.get_block_states().into_iter().for_each(|(k, v)| {
        BLOCK_STATES_BY_ID.insert(*k, v.clone());
    });
    let protocol_id = block.get_block_id();
    BLOCKS_BY_NAME.insert(id.to_string(), protocol_id);
    BLOCKS_BY_ID.insert(protocol_id, block);
}

pub struct BlockBuilder {
    pub protocol_id: u32,
    pub default_state: u32,
    pub block_states: HashMap<u32, Arc<(dyn BlockState)>>,
    pub block_settings: BlockSettings,
}
impl BlockBuilder {
    pub fn new<T: 'static + BlockState + DeserializeOwned>(
        id: &str,
        block_settings: BlockSettings,
    ) -> BlockBuilder {
        let protocol_id = get_protocol_id("minecraft:block", &id).unwrap();
        let (block_states, default_state) = get_block_states::<T>(&id);
        BlockBuilder {
            protocol_id,
            default_state,
            block_states,
            block_settings,
        }
    }
}

pub enum BlockType {
    Solid,
    Liquid,
    Air,
}

pub struct BlockSettings {
    pub hardness: f32,
    pub resistance: f32,
    pub light_level: u8,
    pub block_type: BlockType,
}
impl BlockSettings {
    pub fn new() -> BlockSettings {
        BlockSettings {
            hardness: 6.0,
            resistance: 6.0,
            light_level: 0,
            block_type: BlockType::Solid,
        }
    }
    pub fn hardness(mut self, hardness: f32) -> BlockSettings {
        self.hardness = hardness;
        self
    }
    pub fn resistance(mut self, resistance: f32) -> BlockSettings {
        self.resistance = resistance;
        self
    }
    pub fn light_level(mut self, light_level: u8) -> BlockSettings {
        self.light_level = light_level;
        self
    }
    pub fn block_type(mut self, block_type: BlockType) -> BlockSettings {
        self.block_type = block_type;
        self
    }
    pub fn strength(mut self, strength: f32) -> BlockSettings {
        self.hardness = strength;
        self.resistance = strength;
        self
    }
}

impl Default for BlockSettings {
    fn default() -> Self {
        BlockSettings::new()
    }
}

#[macro_export]
macro_rules! block_state {
    ($name:tt, $properties:ident, $id:expr) => {
        #[derive(Deserialize)]
        pub struct $name {
            pub properties: $properties,
            #[serde(default)]
            pub default: bool,
            pub id: u32,
        }

        impl BlockState for $name {
            fn get_block_id(&self) -> u32 {
                *BLOCKS_BY_NAME.get($id).unwrap()
            }

            fn get_block_state(&self) -> u32 {
                self.id
            }

            fn is_default(&self) -> bool {
                self.default
            }
        }
    };
}

#[macro_export]
macro_rules! empty_block_state {
    ($name:tt, $id:expr) => {
        #[derive(Deserialize)]
        pub struct $name {
            #[serde(default)]
            pub default: bool,
            pub id: u32,
        }

        impl BlockState for $name {
            fn get_block_id(&self) -> u32 {
                *BLOCKS_BY_NAME.get($id).unwrap()
            }

            fn get_block_state(&self) -> u32 {
                self.id
            }

            fn is_default(&self) -> bool {
                self.default
            }
        }
    };
}

#[macro_export]
macro_rules! block_def {
    ($name:tt, $builder:expr) => {
        pub struct $name {
            pub builder: BlockBuilder,
        }
        impl Block for $name {
            fn get_builder(&self) -> &BlockBuilder {
                &self.builder
            }
        }
        impl $name {
            pub(crate) fn new() -> $name {
                $name { builder: $builder }
            }
        }
    };
}
