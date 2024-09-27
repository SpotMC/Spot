use crate::network::connection::Connection;
use crate::network::packet::s2c::finish_configuration::INSTANCE;
use crate::registry::send_registry_data;
use std::io::Error;
use tokio::io::AsyncRead;

pub(crate) async fn known_packs<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    _data: R,
) -> Result<(), Error> {
    send_registry_data(connection).await?;
    connection.send_packet(&INSTANCE).await?;
    Ok(())
}
