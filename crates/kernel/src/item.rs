use crate::block::BLOCK_ITEM_BY_ID;
use crate::registry::protocol_id::get_protocol_id;
use dashmap::DashMap;
use downcast_rs::{impl_downcast, DowncastSync};
use once_cell::sync::Lazy;

pub(crate) static ITEMS_BY_ID: Lazy<DashMap<u32, Box<dyn Item>>> = Lazy::new(DashMap::new);
pub(crate) static ITEMS_BY_NAME: Lazy<DashMap<String, u32>> = Lazy::new(DashMap::new);
pub fn register_item(id: &str, item: Box<dyn Item + 'static>) {
    let protocol_id = item.get_item_id();
    ITEMS_BY_NAME.insert(id.to_string(), protocol_id);
    ITEMS_BY_ID.insert(protocol_id, item);
}
pub fn register_block_item(id: &str, item: Box<dyn Item + 'static>, block: u32) {
    let protocol_id = item.get_item_id();
    BLOCK_ITEM_BY_ID.insert(block, protocol_id);
    ITEMS_BY_NAME.insert(id.to_string(), protocol_id);
    ITEMS_BY_ID.insert(protocol_id, item);
}
pub trait Item: Send + Sync + DowncastSync {
    fn get_builder(&self) -> &ItemBuilder;
    fn get_item_id(&self) -> u32 {
        self.get_builder().protocol_id
    }
}
impl_downcast!(sync Item);

pub trait BlockItem: Item {
    fn get_block() -> u32;
}

pub struct ItemBuilder {
    pub protocol_id: u32,
    pub item_settings: ItemSettings,
}

impl ItemBuilder {
    fn new(id: &str, item_settings: ItemSettings) -> ItemBuilder {
        ItemBuilder {
            protocol_id: get_protocol_id("minecraft:item", &id).unwrap(),
            item_settings,
        }
    }
}

pub struct ItemSettings {
    pub max_count: u8,
    pub max_damage: u16,
    pub fireproof: bool,
}

impl ItemSettings {
    pub fn new() -> ItemSettings {
        ItemSettings {
            max_count: 64,
            max_damage: 0,
            fireproof: false,
        }
    }
    pub fn max_count(mut self, max_count: u8) -> ItemSettings {
        self.max_count = max_count;
        self
    }
    pub fn max_damage(mut self, max_damage: u16) -> ItemSettings {
        self.max_damage = max_damage;
        self
    }
    pub fn fireproof(mut self) -> ItemSettings {
        self.fireproof = true;
        self
    }
}

impl Default for ItemSettings {
    fn default() -> Self {
        Self::new()
    }
}
