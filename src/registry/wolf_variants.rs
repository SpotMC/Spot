use crate::registry::load_static_registries;
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

lazy_static! {
    pub static ref WOLF_VARIANTS: DashMap<String, WolfVariant> =
        load_static_registries("wolf_variants.json", |v: Value| {
            let wolf_variant: WolfVariant = serde_json::from_value(v).unwrap();
            wolf_variant
        });
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WolfVariant {
    pub wild_texture: String,
    pub angry_texture: String,
    pub biomes: String,
    pub tame_texture: String,
}