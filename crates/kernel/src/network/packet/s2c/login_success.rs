use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::util::io::WriteExt;
use crate::util::{write_str, write_var_int};
use anyhow::{anyhow, Result};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(crate) static INSTANCE: LoginSuccessS2C = LoginSuccessS2C {};
pub(crate) struct LoginSuccessS2C {}

impl Encode for LoginSuccessS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<()> {
        if connection.username.is_none() || connection.uuid.is_none() {
            return Err(anyhow!("Login start packet is not accepted yet.",));
        }
        buf.write_u128(connection.uuid.unwrap()).await?;
        write_str(buf, &connection.username.clone().unwrap()).await?;
        write_var_int(buf, 0).await?;
        buf.write_bool(true).await?;
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x02
    }
}
