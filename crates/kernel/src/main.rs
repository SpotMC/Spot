pub mod block;
pub mod config;
pub mod entity;
pub mod item;
pub mod nbt;
pub(crate) mod network;
pub mod registry;
mod test;
pub mod util;
pub mod world;

use crate::config::PORT;
use crate::registry::registries::register_vanilla;
use crate::registry::{
    BIOMES_INDEX, DAMAGE_TYPES_INDEX, DIMENSION_TYPES_INDEX, PAINTING_VARIANTS_INDEX,
    WOLF_VARIANTS_INDEX,
};
use mimalloc::MiMalloc;
use network::connection::read_socket;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use static_files::Resource;
use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::MissedTickBehavior::Skip;
use tracing::{debug, info, instrument, trace};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

pub const PROTOCOL_VERSION: i32 = 767;
pub const MINECRAFT_VERSION: &str = "1.21";
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
//noinspection RsUnresolvedPath
pub static GENERATED: Lazy<HashMap<&'static str, Resource>> = Lazy::new(generate);
pub static mut WORLD: Lazy<RwLock<world::World>> = Lazy::new(|| RwLock::from(world::World::new()));
#[tokio::main]
#[instrument]
async fn main() {
    fmt()
        .event_format(
            format()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(true)
                .with_file(true)
                .with_thread_names(true)
                .with_source_location(true),
        )
        .init();
    tokio::spawn(async {
        loop {
            let mut reader = BufReader::new(stdin());
            let mut buf = String::new();
            reader
                .read_line(&mut buf)
                .await
                .expect("Error in reading commands from stdin.");
        }
    });
    let time = std::time::Instant::now();
    info!("Binding PORT: {:?}.", *PORT);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT))
        .await
        .unwrap();
    info!("Loaded {:?} biomes.", BIOMES_INDEX.len());
    info!("Loaded {:?} dimension types.", DIMENSION_TYPES_INDEX.len());
    info!("Loaded {:?} damage types.", DAMAGE_TYPES_INDEX.len());
    info!("Loaded {:?} wolf variants.", WOLF_VARIANTS_INDEX.len());
    info!(
        "Loaded {:?} painting variants.",
        PAINTING_VARIANTS_INDEX.len()
    );
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        interval.set_missed_tick_behavior(Skip);
        loop {
            unsafe {
                WORLD.write().tick();
            }
            interval.tick().await;
        }
    });
    register_vanilla();
    tokio::spawn(async move {
        info!("Network thread started.");
        info!("Time elapsed {:?} ns", time.elapsed().as_nanos());
        loop {
            accept_connection(listener.accept().await).await;
        }
    })
    .await
    .unwrap();
}

#[instrument]
async fn accept_connection(result: Result<(TcpStream, SocketAddr), Error>) {
    match result {
        Ok((mut socket, _addr)) => {
            tokio::spawn(async move {
                trace!("New connection accepted");
                match read_socket(&mut socket).await {
                    Ok(()) => {}
                    Err(err) => {
                        debug!("Error in handling packet: {:?}", err);
                        let _ = socket.shutdown().await;
                    }
                }
            });
        }
        Err(err) => {
            debug!("Error in accepting connection: {:?}", err)
        }
    }
}
