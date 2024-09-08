use std::io::Error;

use crate::network::connection::Connection;
use crate::network::packet::s2c::login_success;
use crate::util::io::ReadExt;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(crate) async fn login_start<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    mut data: R,
) -> Result<(), Error> {
    connection.username = Some(data.read_str().await?);
    connection.uuid = Some(data.read_u128().await?);
    connection.send_packet(&login_success::INSTANCE).await?;
    Ok(())
}
