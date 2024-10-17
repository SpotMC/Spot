use crate::network::connection::Connection;
use anyhow::Result;
use tokio::io::AsyncWrite;

pub mod c2s;
pub mod s2c;

pub trait Encode {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<()>;
    fn get_id(&self) -> i32;
}
