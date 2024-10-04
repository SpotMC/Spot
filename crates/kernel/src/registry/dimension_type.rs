use crate::registry::{load_static_registries, NbtSerializable};
use dashmap::DashMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spotlight::nbt::*;
use spotlight::{nbt_byte, nbt_double, nbt_float, nbt_int, nbt_long, nbt_str};
use std::sync::LazyLock;

pub static DIMENSION_TYPES: LazyLock<DashMap<String, DimensionType>> = LazyLock::new(|| {
    load_static_registries("dimension_types.json", |v: Value| {
        let dim: DimensionType = serde_json::from_value(v).unwrap();
        dim
    })
});

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionType {
    #[serde(default)]
    pub fixed_time: Option<i64>,
    pub piglin_safe: i8,
    pub natural: i8,
    pub ambient_light: f32,
    pub monster_spawn_block_light_limit: i32,
    pub infiniburn: String,
    pub respawn_anchor_works: i8,
    pub has_skylight: i8,
    pub bed_works: i8,
    pub effects: String,
    pub has_raids: i8,
    pub logical_height: i32,
    pub coordinate_scale: f64,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub min_y: i32,
    pub ultrawarm: i8,
    pub has_ceiling: i8,
    pub height: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonsterSpawnLightLevel {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl NbtSerializable for DimensionType {
    fn to_nbt(&self) -> NbtCompound {
        let data: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(18);
        data.insert("piglin_safe".to_string(), nbt_byte!(self.piglin_safe));
        data.insert("natural".to_string(), nbt_byte!(self.natural));
        data.insert("ambient_light".to_string(), nbt_float!(self.ambient_light));
        data.insert(
            "monster_spawn_block_light_limit".to_string(),
            nbt_int!(self.monster_spawn_block_light_limit),
        );
        data.insert("infiniburn".to_string(), nbt_str!(self.infiniburn.clone()));
        data.insert(
            "respawn_anchor_works".to_string(),
            nbt_byte!(self.respawn_anchor_works),
        );
        data.insert("has_skylight".to_string(), nbt_byte!(self.has_skylight));
        data.insert("bed_works".to_string(), nbt_byte!(self.bed_works));
        data.insert("effects".to_string(), nbt_str!(self.clone().effects));
        data.insert("has_raids".to_string(), nbt_byte!(self.has_raids));
        data.insert("logical_height".to_string(), nbt_int!(self.logical_height));
        data.insert(
            "coordinate_scale".to_string(),
            nbt_double!(self.coordinate_scale),
        );
        let map: DashMap<String, Box<dyn NbtTag>> = DashMap::new();
        map.insert(
            "min_inclusive".to_string(),
            nbt_int!(self.monster_spawn_light_level.min_inclusive),
        );
        map.insert(
            "max_inclusive".to_string(),
            nbt_int!(self.monster_spawn_light_level.max_inclusive),
        );
        map.insert(
            "type".to_string(),
            nbt_str!(self.monster_spawn_light_level.type_field.clone()),
        );
        data.insert(
            "monster_spawn_light_level".to_string(),
            Box::from(NbtCompound { data: map }),
        );
        data.insert("min_y".to_string(), nbt_int!(self.min_y));
        data.insert("ultrawarm".to_string(), nbt_byte!(self.ultrawarm));
        data.insert("has_ceiling".to_string(), nbt_byte!(self.has_ceiling));
        data.insert("height".to_string(), nbt_int!(self.height));
        if self.fixed_time.is_some() {
            data.insert(
                "fixed_time".to_string(),
                nbt_long!(self.fixed_time.unwrap()),
            );
        }
        NbtCompound { data }
    }
}
