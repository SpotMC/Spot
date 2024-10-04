use crate::block::BlockState;
use crate::GENERATED;
use hashbrown::HashMap;
use parking_lot::RwLock;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::{Arc, LazyLock};

pub static REGISTRY: LazyLock<Value> = LazyLock::new(|| get_registry("registries.json"));
pub static BLOCK_STATES: LazyLock<Value> = LazyLock::new(|| get_registry("blocks.json"));

fn get_registry(file: &str) -> Value {
    let json = GENERATED.get(file).unwrap();
    serde_json::from_str(
        String::from_utf8(Vec::from(json.data))
            .unwrap_or_else(|_| panic!("Invalid json: registries.json."))
            .as_str(),
    )
    .unwrap()
}

pub fn get_protocol_id(registry_type: &str, name: &str) -> Option<u32> {
    Some(
        REGISTRY
            .get(registry_type)?
            .as_object()?
            .get("entries")?
            .as_object()?
            .get(name)?
            .as_object()?
            .get("protocol_id")?
            .as_u64()? as u32,
    )
}

pub fn get_block_states<T: 'static + DeserializeOwned + BlockState>(
    identifier: &str,
) -> (HashMap<u32, Arc<(dyn BlockState)>>, u32) {
    let values = BLOCK_STATES
        .get(identifier)
        .unwrap()
        .as_object()
        .unwrap()
        .get("states")
        .unwrap()
        .as_array()
        .unwrap();
    let default = RwLock::from(0);
    let map: RwLock<HashMap<u32, Arc<(dyn BlockState)>>> =
        RwLock::from(HashMap::with_capacity(values.len()));
    values.par_iter().for_each(|v| {
        let v: T = serde_json::from_value(v.clone()).unwrap();
        if v.is_default() {
            let mut t = default.write();
            *t = v.get_block_state();
        }
        map.write().insert(v.get_block_state(), Arc::from(v));
    });
    (map.into_inner(), default.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_protocol_id() {
        assert_eq!(
            get_protocol_id("minecraft:entity_type", "minecraft:player"),
            Some(128)
        );
    }
}
