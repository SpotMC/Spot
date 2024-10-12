use crate::network::connection::{ChatMode, Connection, MainHand};
use crate::util::io::ReadExt;
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(crate) async fn client_information<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    mut data: R,
) -> Result<()> {
    connection.locale = Some(data.read_str().await?);
    connection.view_distance = Some(data.read_i8().await?);
    connection.chat_mode = Some(match data.read_var_int().await? {
        0 => ChatMode::Enabled,
        1 => ChatMode::CommandsOnly,
        _ => ChatMode::Hidden,
    });
    connection.chat_colors = Some(data.read_bool().await?);
    connection.skin_parts = Some(data.read_u8().await?);
    connection.main_hand = Some(match data.read_var_int().await? {
        0 => MainHand::Left,
        _ => MainHand::Right,
    });
    connection.enable_text_filtering = Some(data.read_bool().await?);
    connection.allow_server_listings = Some(data.read_bool().await?);
    Ok(())
}
