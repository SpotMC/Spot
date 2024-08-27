use crate::registry::load_static_registries;
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

lazy_static! {
    pub static ref BIOMES: DashMap<String, Biome> =
        load_static_registries("biomes.json", |v: Value| {
            let biome: Biome = serde_json::from_value(v).unwrap();
            biome
        });
}

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
    pub probability: f64,
    pub options: ParticleOptions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbientSound {
    pub sound_id: String,
    #[serde(default)]
    pub range: Option<f32>,
}
