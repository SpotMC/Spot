use crate::network::connection::{Connection, State};
use crate::network::packet::s2c::known_packs_s2c;
use crate::network::packet::Decode;
use anyhow::Result;
use async_trait::async_trait;

pub struct LoginAcknowledged;

#[async_trait]
impl Decode for LoginAcknowledged {
    async fn decode(&self, connection: &mut Connection<'_>, _data: Vec<u8>) -> Result<()> {
        connection.state = State::Configuration;
        connection.send_packet(&known_packs_s2c::INSTANCE).await?;
        Ok(())
    }
}
