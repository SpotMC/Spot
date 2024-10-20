use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::util::io::WriteExt;
use tokio::io::AsyncWrite;

pub struct SetCenterChunkS2C {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Encode for SetCenterChunkS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> anyhow::Result<()> {
        buf.write_var_int(self.chunk_x).await?;
        buf.write_var_int(self.chunk_z).await?;
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x54
    }
}
