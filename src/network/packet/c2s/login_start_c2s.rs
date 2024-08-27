use std::io::Error;

use crate::network::connection::Connection;
use crate::network::packet::s2c::login_success_s2c;
use crate::read_str;
use crate::util::read_str;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(crate) async fn login_start<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    mut data: R,
) -> Result<(), Error> {
    connection.username = Some(read_str!(data));
    connection.uuid = Some(data.read_u128().await?);
    connection.send_packet(&login_success_s2c::INSTANCE).await?;
    Ok(())
}
