use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::registry::get_cache;
use crate::util::{write_str, write_var_int};
use crate::write_bool;
use dashmap::DashMap;
use serde::Serialize;
use std::io::Error;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

pub(crate) struct RegistryDataS2C<'a, T: Serialize> {
    pub(crate) id: &'a str,
    pub(crate) map: &'a DashMap<String, T>,
    pub(crate) cache: &'a DashMap<String, Vec<u8>>,
}

impl<'a, T: Serialize> Encode for RegistryDataS2C<'a, T> {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<(), Error> {
        write_str(buf, self.id).await?;
        write_var_int(buf, self.map.len() as i32).await?;
        for entry in self.map.iter() {
            write_str(buf, entry.key()).await?;
            write_bool!(buf, true);
            buf.write_all(&get_cache(entry.key(), entry.value(), self.cache)?)
                .await?;
        }
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x07
    }
}
