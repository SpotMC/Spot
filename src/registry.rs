use crate::nbt::{serde_nbt, NbtCompound};
use crate::network::connection::Connection;
use crate::network::packet::s2c::registry_data::RegistryDataS2C;
use crate::GENERATED;
use bytes::BytesMut;
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde_json::Value;
use std::io::Error;
use std::str::FromStr;

pub mod biome;
pub mod damage_type;
pub mod dimension_type;
pub mod painting_variant;
pub mod protocol_id;
pub mod wolf_variant;

#[inline(always)]
pub(crate) fn load_static_registries<F, T>(file: &str, fun: F) -> DashMap<String, T>
where
    F: Fn(Value) -> T,
{
    let buf = GENERATED.get(file).unwrap().data;
    let v = Value::from_str(
        String::from_utf8(Vec::from(buf))
            .unwrap_or_else(|_| panic!("Invalid json {}.", file))
            .as_str(),
    )
    .unwrap_or_else(|_| panic!("Invalid json {}.", file));
    let values = v.as_object().unwrap();
    let map = DashMap::with_capacity(values.len());
    values.iter().for_each(|(key, value)| {
        let t: T = fun(value.to_owned());
        map.insert(key.clone(), t);
    });
    map
}

lazy_static! {
    pub(crate) static ref BIOMES_CACHE: DashMap<String, Vec<u8>> =
        DashMap::with_capacity(biome::BIOMES.len());
    pub(crate) static ref PAINTING_VARIANTS_CACHE: DashMap<String, Vec<u8>> =
        DashMap::with_capacity(painting_variant::PAINTING_VARIANTS.len());
    pub(crate) static ref DAMAGE_TYPES_CACHE: DashMap<String, Vec<u8>> =
        DashMap::with_capacity(damage_type::DAMAGE_TYPES.len());
    pub(crate) static ref WOLF_VARIANTS_CACHE: DashMap<String, Vec<u8>> =
        DashMap::with_capacity(wolf_variant::WOLF_VARIANTS.len());
    pub(crate) static ref DIMENSION_TYPES_CACHE: DashMap<String, Vec<u8>> =
        DashMap::with_capacity(dimension_type::DIMENSION_TYPES.len());
    pub(crate) static ref BIOMES_INDEX: Vec<String> = index_registry_data(&biome::BIOMES);
    pub(crate) static ref PAINTING_VARIANTS_INDEX: Vec<String> =
        index_registry_data(&painting_variant::PAINTING_VARIANTS);
    pub(crate) static ref DAMAGE_TYPES_INDEX: Vec<String> =
        index_registry_data(&damage_type::DAMAGE_TYPES);
    pub(crate) static ref WOLF_VARIANTS_INDEX: Vec<String> =
        index_registry_data(&wolf_variant::WOLF_VARIANTS);
    pub(crate) static ref DIMENSION_TYPES_INDEX: Vec<String> =
        index_registry_data(&dimension_type::DIMENSION_TYPES);
}
#[inline]
pub(crate) fn get_cache<T: NbtSerializable>(
    id: &str,
    raw_data: &T,
    cache: &DashMap<String, Vec<u8>>,
) -> Result<Vec<u8>, Error> {
    let data = cache.get(id);
    Ok(if let Some(data) = data {
        data.value().to_owned()
    } else {
        let mut buf = BytesMut::new();
        serde_nbt(raw_data.to_nbt(), &mut buf);
        let buf = buf.to_vec();
        cache.insert(id.parse().unwrap(), buf.clone());
        buf
    })
}

pub(crate) async fn send_registry_data<'a>(connection: &mut Connection<'a>) -> Result<(), Error> {
    connection
        .send_packet(&RegistryDataS2C {
            id: "minecraft:worldgen/biome",
            map: &biome::BIOMES,
            cache: &BIOMES_CACHE,
            index: &BIOMES_INDEX,
        })
        .await?;
    connection
        .send_packet(&RegistryDataS2C {
            id: "minecraft:painting_variant",
            map: &painting_variant::PAINTING_VARIANTS,
            cache: &PAINTING_VARIANTS_CACHE,
            index: &PAINTING_VARIANTS_INDEX,
        })
        .await?;
    connection
        .send_packet(&RegistryDataS2C {
            id: "minecraft:damage_type",
            map: &damage_type::DAMAGE_TYPES,
            cache: &DAMAGE_TYPES_CACHE,
            index: &DAMAGE_TYPES_INDEX,
        })
        .await?;
    connection
        .send_packet(&RegistryDataS2C {
            id: "minecraft:wolf_variant",
            map: &wolf_variant::WOLF_VARIANTS,
            cache: &WOLF_VARIANTS_CACHE,
            index: &WOLF_VARIANTS_INDEX,
        })
        .await?;
    connection
        .send_packet(&RegistryDataS2C {
            id: "minecraft:dimension_type",
            map: &dimension_type::DIMENSION_TYPES,
            cache: &DIMENSION_TYPES_CACHE,
            index: &DIMENSION_TYPES_INDEX,
        })
        .await?;
    Ok(())
}

pub trait NbtSerializable {
    fn to_nbt(&self) -> NbtCompound;
}

pub(crate) fn index_registry_data<T>(map: &DashMap<String, T>) -> Vec<String> {
    let mut vec = Vec::with_capacity(map.len());
    map.iter().for_each(|entry| vec.push(entry.key().clone()));
    vec
}
