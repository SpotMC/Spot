use crate::nbt::{NbtCompound, NbtString, NbtTag};
use crate::nbt_str;
use crate::registry::{load_static_registries, NbtSerializable};
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

impl NbtSerializable for WolfVariant {
    fn to_nbt(&self) -> NbtCompound {
        let data: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(4);
        data.insert(
            "wild_texture".to_string(),
            nbt_str!(self.wild_texture.clone()),
        );
        data.insert(
            "angry_texture".to_string(),
            nbt_str!(self.angry_texture.clone()),
        );
        data.insert("biomes".to_string(), nbt_str!(self.biomes.clone()));
        data.insert(
            "tame_texture".to_string(),
            nbt_str!(self.tame_texture.clone()),
        );
        NbtCompound { data }
    }
}
