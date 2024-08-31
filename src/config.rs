use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};

pub static mut MAX_PLAYERS: i32 = 64;
pub static mut VIEW_DISTANCE: i32 = 16;
pub static mut SIMULATION_DISTANCE: i32 = 16;
pub static mut SEED: i64 = 0;
pub static HASHED_SEED: Lazy<i64> = Lazy::new(|| {
    let mut sha = Sha256::new();
    unsafe {
        sha.update(SEED.to_be_bytes());
    }
    let hash_result = sha.finalize();
    let bytes_array: [u8; 8] = hash_result[..8].to_vec().try_into().unwrap();
    i64::from_be_bytes(bytes_array)
});
