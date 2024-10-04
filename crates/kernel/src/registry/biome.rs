use crate::registry::{load_static_registries, NbtSerializable};
use dashmap::DashMap;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use spotlight::nbt::*;
use spotlight::{nbt_byte, nbt_double, nbt_float, nbt_int, nbt_str};
use std::sync::LazyLock;

pub static BIOMES: LazyLock<DashMap<String, Biome>> = LazyLock::new(|| {
    load_static_registries("biomes.json", |v: Value| {
        let biome: Biome = serde_json::from_value(v).unwrap();
        biome
    })
});

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Biome {
    pub effects: Effects,
    pub has_precipitation: i8,
    pub temperature: f32,
    pub downfall: f32,
    #[serde(default)]
    pub temperature_modifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effects {
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
    pub sky_color: i32,
    #[serde(default)]
    pub grass_color: Option<i32>,
    #[serde(default)]
    pub grass_color_modifier: Option<String>,
    #[serde(default)]
    pub foliage_color: Option<i32>,
    #[serde(default)]
    pub music: Option<Music>,
    #[serde(default)]
    pub mood_sound: Option<MoodSound>,
    #[serde(default)]
    pub additions_sound: Option<AdditionsSound>,
    #[serde(default)]
    pub particle: Option<Particle>,
    #[serde(default)]
    pub ambient_sound: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Music {
    pub replace_current_music: i8,
    pub max_delay: i32,
    pub sound: String,
    pub min_delay: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoodSound {
    pub tick_delay: i32,
    pub offset: f64,
    pub sound: String,
    pub block_search_extent: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Particle {
    pub probability: f32,
    pub options: ParticleOptions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}

impl NbtSerializable for Biome {
    fn to_nbt(&self) -> NbtCompound {
        let map: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(5);
        let effects: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(12);
        map.insert(
            "has_precipitation".to_string(),
            nbt_byte!(self.has_precipitation),
        );
        map.insert("temperature".to_string(), nbt_float!(self.temperature));
        map.insert("downfall".to_string(), nbt_float!(self.downfall));
        effects.insert(
            "water_fog_color".to_string(),
            nbt_int!(self.effects.water_fog_color),
        );
        effects.insert("fog_color".to_string(), nbt_int!(self.effects.fog_color));
        effects.insert(
            "water_color".to_string(),
            nbt_int!(self.effects.water_color),
        );
        effects.insert("sky_color".to_string(), nbt_int!(self.effects.sky_color));
        if self.temperature_modifier.is_some() {
            map.insert(
                "temperature_modifier".to_string(),
                nbt_str!(self.temperature_modifier.clone().unwrap()),
            );
        }
        if self.effects.grass_color.is_some() {
            effects.insert(
                "grass_color".to_string(),
                nbt_int!(self.effects.grass_color.unwrap()),
            );
        }
        if self.effects.grass_color_modifier.is_some() {
            effects.insert(
                "grass_color_modifier".to_string(),
                nbt_str!(self.effects.grass_color_modifier.clone().unwrap()),
            );
        }
        if self.effects.foliage_color.is_some() {
            effects.insert(
                "foliage_color".to_string(),
                nbt_int!(self.effects.foliage_color.unwrap()),
            );
        }
        if self.effects.music.is_some() {
            let map: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(4);
            let music = self.effects.music.clone().unwrap();
            map.insert(
                "replace_current_music".to_string(),
                nbt_byte!(music.replace_current_music),
            );
            map.insert("max_delay".to_string(), nbt_int!(music.max_delay));
            map.insert("sound".to_string(), nbt_str!(music.sound));
            map.insert("min_delay".to_string(), nbt_int!(music.min_delay));
            effects.insert("music".to_string(), Box::from(NbtCompound { data: map }));
        }
        if self.effects.mood_sound.is_some() {
            let map: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(4);
            let mood_sound = self.effects.mood_sound.clone().unwrap();
            map.insert("tick_delay".to_string(), nbt_int!(mood_sound.tick_delay));
            map.insert("offset".to_string(), nbt_double!(mood_sound.offset));
            map.insert("sound".to_string(), nbt_str!(mood_sound.sound));
            map.insert(
                "block_search_extent".to_string(),
                nbt_int!(mood_sound.block_search_extent),
            );
            effects.insert(
                "mood_sound".to_string(),
                Box::from(NbtCompound { data: map }),
            );
        }
        if self.effects.additions_sound.is_some() {
            let map: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(2);
            let additions_sound = self.effects.additions_sound.clone().unwrap();
            map.insert("sound".to_string(), nbt_str!(additions_sound.sound));
            map.insert(
                "tick_chance".to_string(),
                nbt_double!(additions_sound.tick_chance),
            );
            effects.insert(
                "additions_sound".to_string(),
                Box::from(NbtCompound { data: map }),
            );
        }
        if self.effects.particle.is_some() {
            let map: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(2);
            let particle = self.effects.particle.clone().unwrap();
            map.insert("probability".to_string(), nbt_float!(particle.probability));
            let particle_options: DashMap<String, Box<dyn NbtTag>> = DashMap::with_capacity(1);
            particle_options.insert("type".to_string(), nbt_str!(particle.options.particle_type));
            map.insert(
                "options".to_string(),
                Box::from(NbtCompound {
                    data: particle_options,
                }),
            );
            effects.insert("particle".to_string(), Box::from(NbtCompound { data: map }));
        }
        if self.effects.ambient_sound.is_some() {
            effects.insert(
                "ambient_sound".to_string(),
                nbt_str!(self.effects.ambient_sound.clone().unwrap()),
            );
        }
        map.insert(
            "effects".to_string(),
            Box::from(NbtCompound { data: effects }),
        );
        NbtCompound { data: map }
    }
}
