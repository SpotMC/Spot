use crate::network::connection::Connection;
use crate::network::packet::Encode;
use std::io::Error;
use tokio::io::AsyncWrite;

pub(crate) static INSTANCE: FinishConfigurationS2C = FinishConfigurationS2C {};

pub(crate) struct FinishConfigurationS2C {}

impl Encode for FinishConfigurationS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        _buf: &mut W,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x03
    }
}
