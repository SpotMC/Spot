use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::registry;
use crate::registry::get_cache;
use crate::util::io::WriteExt;
use crate::util::{write_str, write_var_int};
use anyhow::Result;
use dashmap::DashMap;
use serde::Serialize;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

pub struct RegistryDataS2C<'a, T: Serialize> {
    pub(crate) id: &'a str,
    pub(crate) map: &'a DashMap<String, T>,
    pub(crate) cache: &'a DashMap<String, Vec<u8>>,
    pub(crate) index: &'a Vec<String>,
}

impl<'a, T: Serialize + registry::NbtSerializable> Encode for RegistryDataS2C<'a, T> {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<()> {
        write_str(buf, self.id).await?;
        write_var_int(buf, self.map.len() as i32).await?;
        for key in self.index {
            let entry = self.map.get_mut(key).unwrap();
            write_str(buf, entry.key()).await?;
            buf.write_bool(true).await?;
            buf.write_all(&get_cache(entry.key(), entry.value(), self.cache)?)
                .await?;
        }
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x07
    }
}
