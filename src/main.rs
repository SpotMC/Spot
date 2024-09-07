pub mod block;
pub mod config;
pub mod entity;
pub mod nbt;
pub(crate) mod network;
pub mod registry;
pub mod util;
pub mod world;

use crate::config::PORT;
use crate::registry::{
    BIOMES_INDEX, DAMAGE_TYPES_INDEX, DIMENSION_TYPES_INDEX, PAINTING_VARIANTS_INDEX,
    WOLF_VARIANTS_INDEX,
};
use mimalloc::MiMalloc;
use network::connection::read_socket;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use static_files::Resource;
use std::collections::HashMap;
use tklog::{async_debug, async_info, async_trace};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::MissedTickBehavior::Skip;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

pub const PROTOCOL_VERSION: i32 = 767;
pub const MINECRAFT_VERSION: &str = "1.21";
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
pub static GENERATED: Lazy<HashMap<&'static str, Resource>> = Lazy::new(generate);
pub static mut WORLD: Lazy<Mutex<world::World>> = Lazy::new(|| Mutex::from(world::World::new()));
pub static mut STOPPED: bool = false;
#[tokio::main]
async fn main() {
    let time = std::time::Instant::now();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT))
        .await
        .unwrap();
    async_info!("Binding PORT: ", *PORT);
    async_info!("Loaded ", BIOMES_INDEX.len(), " biomes.");
    async_info!("Loaded ", DIMENSION_TYPES_INDEX.len(), " dimension types.");
    async_info!("Loaded ", DAMAGE_TYPES_INDEX.len(), " damage types.");
    async_info!("Loaded ", WOLF_VARIANTS_INDEX.len(), " wolf variants.");
    async_info!(
        "Loaded ",
        PAINTING_VARIANTS_INDEX.len(),
        " painting variants."
    );
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        interval.set_missed_tick_behavior(Skip);
        loop {
            unsafe {
                if STOPPED {
                    break;
                }
            }
            unsafe {
                WORLD.get_mut().tick();
            }
            interval.tick().await;
        }
    });
    tokio::spawn(async move {
        async_info!("Network thread started.");
        async_info!("Time elapsed ", time.elapsed().as_nanos(), " ns");
        loop {
            unsafe {
                if STOPPED {
                    break;
                }
            }
            match listener.accept().await {
                Ok((mut socket, _addr)) => {
                    tokio::spawn(async move {
                        async_trace!("New connection accepted");
                        match read_socket(&mut socket).await {
                            Ok(()) => {}
                            Err(err) => {
                                async_debug!("Error in handling packet: ", err);
                                let _ = socket.shutdown().await;
                            }
                        }
                    });
                }
                Err(err) => {
                    async_debug!("Error in accepting connection: ", err)
                }
            }
        }
    })
    .await
    .unwrap();
}
