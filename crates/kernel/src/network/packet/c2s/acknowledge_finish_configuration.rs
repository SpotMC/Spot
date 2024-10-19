use crate::gameplay::player_join;
use crate::network::connection::Connection;
use anyhow::Result;
use async_trait::async_trait;

pub struct AcknowledgeFinishConfiguration;
#[async_trait]
impl crate::network::packet::Decode for AcknowledgeFinishConfiguration {
    async fn decode(&self, connection: &mut Connection<'_>, _data: Vec<u8>) -> Result<()> {
        player_join(connection).await
    }
}
