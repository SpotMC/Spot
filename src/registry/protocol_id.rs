use crate::GENERATED;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    pub static ref REGISTRY: Value = get_registry("registries.json");
    pub static ref BLOCK_STATES: Value = get_registry("blocks.json");
}
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
