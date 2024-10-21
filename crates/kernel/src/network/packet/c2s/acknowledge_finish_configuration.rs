use crate::gameplay::player_join;
use crate::network::connection::Connection;
use crate::network::connection::State::Play;
use anyhow::Result;
use async_trait::async_trait;

pub struct AcknowledgeFinishConfiguration;
#[async_trait]
impl crate::network::packet::Decode for AcknowledgeFinishConfiguration {
    async fn decode(&self, connection: &mut Connection<'_>, _data: &[u8]) -> Result<()> {
        connection.state = Play;
        player_join(connection).await
    }
}
