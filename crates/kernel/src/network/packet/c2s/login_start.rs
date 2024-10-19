use crate::network::connection::Connection;
use crate::network::packet::s2c::login_success::LoginSuccessS2C;
use crate::network::packet::Decode;
use crate::util::io::ReadExt;
use anyhow::Result;
use async_trait::async_trait;
use tokio::io::AsyncReadExt;

pub struct LoginStart;

#[async_trait]
impl Decode for LoginStart {
    async fn decode(&self, connection: &mut Connection<'_>, data: Vec<u8>) -> Result<()> {
        let mut data = data.as_slice();
        connection.username = Some(data.read_str().await?);
        connection.uuid = Some(data.read_u128().await?);
        connection.send_packet(&LoginSuccessS2C).await?;
        Ok(())
    }
}
