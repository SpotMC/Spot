use crate::network::connection::Connection;
use std::io::Error;
use tokio::io::AsyncWrite;

pub(crate) mod c2s;
pub(crate) mod s2c;

pub(crate) trait Encode {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<(), Error>;
    fn get_id(&self) -> i32;
}
