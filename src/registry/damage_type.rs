use crate::registry::load_static_registries;
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

lazy_static! {
    pub static ref DAMAGE_TYPES: DashMap<String, DamageType> =
        load_static_registries("damage_types.json", |v: Value| {
            let damage_type: DamageType = serde_json::from_value(v).unwrap();
            damage_type
        });
}
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
