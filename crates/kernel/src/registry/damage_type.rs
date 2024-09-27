use crate::nbt::*;
use crate::registry::{load_static_registries, NbtSerializable};
use crate::{nbt_double, nbt_str};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub static DAMAGE_TYPES: Lazy<DashMap<String, DamageType>> = Lazy::new(|| {
    load_static_registries("damage_types.json", |v: Value| {
        let damage_type: DamageType = serde_json::from_value(v).unwrap();
        damage_type
    })
});
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageType {
    #[serde(default)]
    pub effects: Option<String>,
    pub scaling: String,
    pub exhaustion: f64,
    pub message_id: String,
    #[serde(default)]
    pub death_message_type: Option<String>,
}

impl NbtSerializable for DamageType {
    fn to_nbt(&self) -> NbtCompound {
        let data: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(5);
        if self.effects.is_some() {
            data.insert(
                "effects".to_string(),
                nbt_str!(self.effects.clone().unwrap()),
            );
        }
        data.insert("scaling".to_string(), nbt_str!(self.scaling.clone()));
        data.insert("exhaustion".to_string(), nbt_double!(self.exhaustion));
        data.insert("message_id".to_string(), nbt_str!(self.message_id.clone()));
        if self.death_message_type.is_some() {
            data.insert(
                "death_message_type".to_string(),
                nbt_str!(self.death_message_type.clone().unwrap()),
            );
        }
        NbtCompound { data }
    }
}
