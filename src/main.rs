mod nbt;
pub(crate) mod network;
pub mod registry;
pub mod util;

use crate::registry::biome::BIOMES;
use crate::registry::damage_type::DAMAGE_TYPES;
use crate::registry::dimension_type::DIMENSION_TYPES;
use crate::registry::painting_variant::PAINTING_VARIANTS;
use crate::registry::wolf_variant::WOLF_VARIANTS;
use lazy_static::lazy_static;
use network::connection::read_socket;
use static_files::Resource;
use std::collections::HashMap;
use tklog::{async_debug, async_info, async_trace};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub const PROTOCOL_VERSION: i32 = 767;
pub const MINECRAFT_VERSION: &str = "1.21";
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
lazy_static! {
    pub static ref GENERATED: HashMap<&'static str, Resource> = generate();
}
#[tokio::main]
async fn main() {
    let time = std::time::Instant::now();
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    async_info!("Loaded ", BIOMES.len(), " biomes.");
    async_info!("Loaded ", DIMENSION_TYPES.len(), " dimension types.");
    async_info!("Loaded ", DAMAGE_TYPES.len(), " damage types.");
    async_info!("Loaded ", WOLF_VARIANTS.len(), " wolf variants.");
    async_info!("Loaded ", PAINTING_VARIANTS.len(), " painting variants.");
    tokio::spawn(async move {
        async_info!("Network thread started.");
        async_info!("Used ", time.elapsed().as_nanos(), " ns");
        loop {
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

#[test]
fn data_generate() {}
