use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::util::{write_str, write_var_int};
use crate::MINECRAFT_VERSION;
use std::io::Error;
use tokio::io::AsyncWrite;

pub(crate) static INSTANCE: ConfigKnownPacksS2C = ConfigKnownPacksS2C {};
pub(crate) struct ConfigKnownPacksS2C {}

impl Encode for ConfigKnownPacksS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<(), Error> {
        write_var_int(buf, 1).await?;
        write_str(buf, "minecraft").await?;
        write_str(buf, "core").await?;
        write_str(buf, MINECRAFT_VERSION).await?;
        Ok(())
    }
    fn get_id(&self) -> i32 {
        0x0E
    }
}
