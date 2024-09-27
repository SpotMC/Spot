use crate::config::{HASHED_SEED, MAX_PLAYERS, SIMULATION_DISTANCE, VIEW_DISTANCE};
use crate::entity::player::Player;
use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::registry::dimension_type::DIMENSION_TYPES;
use crate::registry::DIMENSION_TYPES_INDEX;
use crate::util::io::WriteExt;
use crate::util::{encode_position, write_str, write_var_int};
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct PlayLoginS2C<'a> {
    pub player: &'a Player,
}

impl Encode for PlayLoginS2C<'_> {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<(), Error> {
        buf.write_i32(self.player.entity.entity_id).await?;
        buf.write_bool(false).await?;
        write_var_int(buf, DIMENSION_TYPES.len() as i32).await?;
        for dimension_type in DIMENSION_TYPES_INDEX.iter() {
            write_str(buf, dimension_type).await?;
        }
        write_var_int(buf, *MAX_PLAYERS).await?;
        write_var_int(buf, *VIEW_DISTANCE).await?;
        write_var_int(buf, *SIMULATION_DISTANCE).await?;
        buf.write_bool(false).await?;
        buf.write_bool(true).await?;
        buf.write_bool(false).await?;
        write_var_int(buf, self.player.entity.dimension as i32).await?;
        write_str(
            buf,
            match DIMENSION_TYPES_INDEX.get(self.player.entity.dimension) {
                Some(index) => index,
                None => return Err(Error::new(ErrorKind::Other, "Dimension type not found")),
            },
        )
        .await?;
        buf.write_i64(*HASHED_SEED).await?;
        buf.write_u8(self.player.game_mode).await?;
        buf.write_i8(self.player.previous_game_mode).await?;
        buf.write_bool(false).await?;
        buf.write_bool(false).await?;
        match &self.player.death_location {
            Some(death_location) => {
                buf.write_bool(true).await?;
                write_str(buf, &death_location.0).await?;
                buf.write_u64(encode_position(
                    death_location.1,
                    death_location.2,
                    death_location.3,
                ))
                .await?;
            }
            None => {
                buf.write_bool(false).await?;
            }
        }
        write_var_int(buf, self.player.entity.portal_cooldown).await?;
        buf.write_bool(false).await?;
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x2B
    }
}
