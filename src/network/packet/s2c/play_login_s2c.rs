use crate::config::{HASHED_SEED, MAX_PLAYERS, SIMULATION_DISTANCE, VIEW_DISTANCE};
use crate::entity::player::Player;
use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::registry::dimension_type::DIMENSION_TYPES;
use crate::registry::DIMENSION_TYPES_INDEX;
use crate::util::{encode_position, write_str, write_var_int};
use crate::write_bool;
use rayon::prelude::*;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct PlayLoginS2C<'a> {
    pub player: &'a Player<'a>,
}

impl Encode for PlayLoginS2C<'_> {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<(), Error> {
        buf.write_i32(self.player.entity_id).await?;
        write_bool!(buf, false);
        write_var_int(buf, DIMENSION_TYPES.len() as i32).await?;
        for dimension_type in DIMENSION_TYPES_INDEX.iter() {
            write_str(buf, dimension_type).await?;
        }
        unsafe {
            write_var_int(buf, MAX_PLAYERS).await?;
            write_var_int(buf, VIEW_DISTANCE).await?;
            write_var_int(buf, SIMULATION_DISTANCE).await?;
        }
        write_bool!(buf, false);
        write_bool!(buf, true);
        write_bool!(buf, false);
        write_var_int(
            buf,
            match DIMENSION_TYPES_INDEX
                .par_iter()
                .position_any(|it| it.eq(&self.player.dimension.dimension_name))
            {
                Some(index) => index as i32,
                None => return Err(Error::new(ErrorKind::Other, "Dimension type not found")),
            },
        )
        .await?;
        write_str(buf, &self.player.dimension.dimension_name).await?;
        buf.write_i64(*HASHED_SEED).await?;
        buf.write_u8(self.player.game_mode).await?;
        buf.write_i8(self.player.previous_game_mode).await?;
        match &self.player.death_location {
            Some(death_location) => {
                write_bool!(buf, true);
                write_str(buf, &death_location.0).await?;
                buf.write_u64(encode_position(
                    death_location.1,
                    death_location.2,
                    death_location.3,
                ))
                .await?;
            }
            None => {
                write_bool!(buf, false);
            }
        }
        write_var_int(buf, self.player.portal_cooldown).await?;
        write_bool!(buf, false);
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x2B
    }
}
