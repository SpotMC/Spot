use crate::GENERATED;
use dashmap::DashMap;
use serde_json::Value;
use std::str::FromStr;

pub mod biome;
pub mod damage_type;
pub mod wolf_variants;

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

