use sha2::{Digest, Sha256};
use std::sync::LazyLock;
use toml::{toml, Value};

static DEFAULT: LazyLock<Value> = LazyLock::new(|| {
    Value::from(toml! {
        max-players = 20
        view-distance = 12
        simulation-distance = 16
        seed = 0
        port = 25565
        worldgen-implementation = "super_flat"
    })
});
static TOML: LazyLock<Value> = LazyLock::new(|| {
    if let Ok(file) = std::fs::read_to_string("./config.toml") {
        toml::from_str(&file).unwrap()
    } else {
        std::fs::write("./config.toml", toml::to_string_pretty(&*DEFAULT).unwrap()).unwrap();
        DEFAULT.clone()
    }
});
pub static WORLDGEN_IMPLEMENTATION: LazyLock<&str> = LazyLock::new(|| {
    TOML.get("worldgen-implementation")
        .unwrap_or_else(|| DEFAULT.get("worldgen-implementation").unwrap())
        .as_str()
        .unwrap()
});
pub static PORT: LazyLock<i32> = LazyLock::new(|| {
    TOML.get("port")
        .unwrap_or_else(|| DEFAULT.get("port").unwrap())
        .as_integer()
        .unwrap() as i32
});
pub static MAX_PLAYERS: LazyLock<i32> = LazyLock::new(|| {
    TOML.get("max-players")
        .unwrap_or_else(|| DEFAULT.get("max-players").unwrap())
        .as_integer()
        .unwrap() as i32
});
pub static VIEW_DISTANCE: LazyLock<i32> = LazyLock::new(|| {
    TOML.get("view-distance")
        .unwrap_or_else(|| DEFAULT.get("view-distance").unwrap())
        .as_integer()
        .unwrap() as i32
});
pub static SIMULATION_DISTANCE: LazyLock<i32> = LazyLock::new(|| {
    TOML.get("simulation-distance")
        .unwrap_or_else(|| DEFAULT.get("simulation-distance").unwrap())
        .as_integer()
        .unwrap() as i32
});
pub static SEED: LazyLock<i64> = LazyLock::new(|| {
    TOML.get("seed")
        .unwrap_or_else(|| DEFAULT.get("seed").unwrap())
        .as_integer()
        .unwrap()
});
pub static HASHED_SEED: LazyLock<i64> = LazyLock::new(|| {
    let mut sha = Sha256::new();
    sha.update(SEED.to_be_bytes());
    let hash_result = sha.finalize();
    let bytes_array: [u8; 8] = hash_result[..8].to_vec().try_into().unwrap();
    i64::from_be_bytes(bytes_array)
});
