use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use toml::{toml, Value};

static TOML: Lazy<Value> = Lazy::new(|| {
    if let Ok(file) = std::fs::read_to_string("./config.toml") {
        toml::from_str(&file).unwrap()
    } else {
        let toml = Value::from(toml! {
            max-players = 20
            view-distance = 12
            simulation-distance = 16
            seed = 0
            port = 25565
        });
        std::fs::write("./config.toml", toml::to_string_pretty(&toml).unwrap()).unwrap();
        toml
    }
});
pub static PORT: Lazy<i32> = Lazy::new(|| TOML.get("port").unwrap().as_integer().unwrap() as i32);
pub static MAX_PLAYERS: Lazy<i32> =
    Lazy::new(|| TOML.get("max-players").unwrap().as_integer().unwrap() as i32);
pub static VIEW_DISTANCE: Lazy<i32> =
    Lazy::new(|| TOML.get("view-distance").unwrap().as_integer().unwrap() as i32);
pub static SIMULATION_DISTANCE: Lazy<i32> = Lazy::new(|| {
    TOML.get("simulation-distance")
        .unwrap()
        .as_integer()
        .unwrap() as i32
});
pub static SEED: Lazy<i64> = Lazy::new(|| TOML.get("seed").unwrap().as_integer().unwrap());
pub static HASHED_SEED: Lazy<i64> = Lazy::new(|| {
    let mut sha = Sha256::new();
    sha.update(SEED.to_be_bytes());
    let hash_result = sha.finalize();
    let bytes_array: [u8; 8] = hash_result[..8].to_vec().try_into().unwrap();
    i64::from_be_bytes(bytes_array)
});
