use crate::network::connection::Connection;
use crate::network::packet::Decode;
use crate::util::io::ReadExt;
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;

pub struct ConfirmTeleportation;

#[async_trait]
impl Decode for ConfirmTeleportation {
    async fn decode(&self, connection: &mut Connection<'_>, mut data: &[u8]) -> Result<()> {
        let read_teleport_id = data.read_var_int().await?;
        let p = connection.player.clone().ok_or(anyhow!(
            "PacketC2S: ConfirmTeleportation: invalid context: player is undefined"
        ))?;
        let mut player = p.lock();
        let teleport_id = match player.teleport_id {
            Some(id) => id,
            None => return Ok(()),
        };
        if read_teleport_id == teleport_id {
            player.teleport_id = None;
        }
        Ok(())
    }
}
