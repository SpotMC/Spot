use crate::network::connection::{Connection, State};
use crate::network::packet::s2c::known_packs_s2c;
use anyhow::Result;
use tokio::io::AsyncRead;

pub(crate) async fn login_acknowledged<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    _data: R,
) -> Result<()> {
    connection.state = State::Configuration;
    connection.send_packet(&known_packs_s2c::INSTANCE).await?;
    Ok(())
}
