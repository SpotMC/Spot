use crate::network::connection::Connection;
use std::io::Error;
use tokio::io::AsyncRead;

pub(crate) async fn known_packs<R: AsyncRead + Unpin>(
    _connection: &mut Connection<'_>,
    _data: R,
) -> Result<(), Error> {
    Ok(())
}
