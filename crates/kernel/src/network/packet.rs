use crate::network::connection::Connection;
use anyhow::Result;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use hashbrown::HashMap;
use std::future::Future;
use std::sync::{Arc, LazyLock};
use tokio::io::AsyncWrite;

pub mod c2s;
pub mod s2c;

pub trait Encode {
    fn encode<W: AsyncWrite + Unpin>(
        &self,
        connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> impl Future<Output = Result<()>>;
    fn get_id(&self) -> i32;
}

#[async_trait]
pub trait Decode: Send + Sync {
    async fn decode(&self, connection: &mut Connection<'_>, data: Vec<u8>) -> Result<()>;
}

pub static LOGIN_DECODERS: LazyLock<ArcSwap<HashMap<i32, Box<dyn Decode>>>> = LazyLock::new(|| {
    let mut map: HashMap<i32, Box<dyn Decode>> = HashMap::new();
    map.insert(0x00, Box::new(c2s::login_start::LoginStart));
    map.insert(0x03, Box::new(c2s::login_acknowledged::LoginAcknowledged));
    ArcSwap::new(Arc::new(map))
});
pub static CONFIGURATION_DECODERS: LazyLock<ArcSwap<HashMap<i32, Box<dyn Decode>>>> =
    LazyLock::new(|| {
        let mut map: HashMap<i32, Box<dyn Decode>> = HashMap::new();
        map.insert(
            0x00,
            Box::new(c2s::acknowledge_finish_configuration::AcknowledgeFinishConfiguration),
        );
        map.insert(0x03, Box::new(c2s::client_info::ClientInformation));
        map.insert(0x07, Box::new(c2s::known_packs_c2s::ServerBoundKnownPacks));
        ArcSwap::new(Arc::new(HashMap::new()))
    });
pub static PLAY_DECODERS: LazyLock<ArcSwap<HashMap<i32, Box<dyn Decode>>>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(HashMap::new())));
