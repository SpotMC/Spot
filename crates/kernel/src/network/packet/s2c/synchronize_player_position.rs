use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::util::io::WriteExt;
use anyhow::anyhow;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct SynchronizePlayerPositionS2C;

impl Encode for SynchronizePlayerPositionS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> anyhow::Result<()> {
        let ((x, y, z), yaw, pitch);
        let teleport_id;
        {
            teleport_id = fastrand::i32(i32::MIN..i32::MAX);
            let p = connection.player.clone().ok_or(anyhow!(
                "PacketS2C: SynchronizePlayerPosition: invalid context: player is undefined"
            ))?;
            let mut player = p.lock();
            player.teleport_id = Some(teleport_id);
            ((x, y, z), yaw, pitch) = (player.entity.pos, player.entity.yaw, player.entity.pitch);
        }

        buf.write_f64(x).await?;
        buf.write_f64(y).await?;
        buf.write_f64(z).await?;
        buf.write_f32(yaw).await?;
        buf.write_f32(pitch).await?;
        buf.write_i8(0).await?;
        buf.write_var_int(teleport_id).await?;
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x40
    }
}
