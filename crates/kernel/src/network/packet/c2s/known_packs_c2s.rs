use crate::network::connection::Connection;
use crate::network::packet::s2c::finish_configuration::INSTANCE;
use crate::network::packet::Decode;
use crate::registry::send_registry_data;
use anyhow::Result;
use async_trait::async_trait;

pub struct ServerBoundKnownPacks;

#[async_trait]
impl Decode for ServerBoundKnownPacks {
    async fn decode(&self, connection: &mut Connection<'_>, _data: Vec<u8>) -> Result<()> {
        send_registry_data(connection).await?;
        connection.send_packet(&INSTANCE).await?;
        Ok(())
    }
}
