use crate::network::connection::Connection;
use crate::network::connection::State::Play;
use std::io::Error;
use tokio::io::AsyncRead;

pub(crate) async fn acknowledge_finish_configuration<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    _data: R,
) -> Result<(), Error> {
    connection.state = Play;
    Ok(())
}
