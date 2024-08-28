use crate::network::connection::{ChatMode, Connection, MainHand};
use crate::util::{read_str, read_var_int};
use crate::{read_bool, read_str, read_var_int};
use std::io::Error;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(crate) async fn client_information<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    mut data: R,
) -> Result<(), Error> {
    connection.locale = Some(read_str!(data));
    connection.view_distance = Some(data.read_i8().await?);
    connection.chat_mode = Some(match read_var_int!(data) {
        0 => ChatMode::Enabled,
        1 => ChatMode::CommandsOnly,
        _ => ChatMode::Hidden,
    });
    connection.chat_colors = Some(read_bool!(data));
    connection.skin_parts = Some(data.read_u8().await?);
    connection.main_hand = Some(match read_var_int!(data) {
        0 => MainHand::Left,
        _ => MainHand::Right,
    });
    connection.enable_text_filtering = Some(read_bool!(data));
    connection.allow_server_listings = Some(read_bool!(data));
    Ok(())
}
