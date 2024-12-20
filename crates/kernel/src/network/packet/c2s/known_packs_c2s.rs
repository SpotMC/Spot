use crate::network::connection::Connection;
use crate::network::packet::s2c::finish_configuration::FinishConfigurationS2C;
use crate::network::packet::Decode;
use crate::registry::send_registry_data;
use anyhow::Result;
use async_trait::async_trait;

pub struct ServerBoundKnownPacks;

#[async_trait]
impl Decode for ServerBoundKnownPacks {
    async fn decode(&self, connection: &mut Connection<'_>, _data: &[u8]) -> Result<()> {
        send_registry_data(connection).await?;
        connection.send_packet(&FinishConfigurationS2C).await?;
        Ok(())
    }
}
