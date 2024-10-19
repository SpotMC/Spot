use crate::network::connection::Connection;
use crate::network::packet::Encode;
use anyhow::Result;
use tokio::io::AsyncWrite;

pub struct FinishConfigurationS2C;

impl Encode for FinishConfigurationS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        _buf: &mut W,
    ) -> Result<()> {
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x03
    }
}
