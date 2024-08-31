use crypto::digest::Digest;
use crypto::sha2::Sha256;
use once_cell::sync::Lazy;

pub static mut MAX_PLAYERS: i32 = 64;
pub static mut VIEW_DISTANCE: i32 = 16;
pub static mut SIMULATION_DISTANCE: i32 = 16;
pub static mut SEED: i64 = 0;
pub static HASHED_SEED: Lazy<i64> = Lazy::new(|| {
    let mut sha = Sha256::new();
    unsafe {
        sha.input(SEED.to_be_bytes().as_slice());
    }
    let mut hashed_seed = [0u8; 8];
    sha.result(&mut hashed_seed);
    i64::from_be_bytes(hashed_seed)
});
