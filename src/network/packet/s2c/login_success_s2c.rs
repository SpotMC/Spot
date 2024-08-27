use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::util::{write_str, write_var_int};
use crate::write_bool;
use std::io::Error;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(crate) static INSTANCE: LoginSuccessS2C = LoginSuccessS2C {};
pub(crate) struct LoginSuccessS2C {}

impl Encode for LoginSuccessS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<(), Error> {
        if connection.username.is_none() || connection.uuid.is_none() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Login start packet is not accepted yet.",
            ));
        }
        buf.write_u128(connection.uuid.unwrap()).await?;
        write_str(buf, &connection.username.clone().unwrap()).await?;
        write_var_int(buf, 0).await?;
        write_bool!(buf, true);
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x02
    }
}
