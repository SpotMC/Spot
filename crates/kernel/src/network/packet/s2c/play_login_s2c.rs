use crate::config::{HASHED_SEED, MAX_PLAYERS, SIMULATION_DISTANCE, VIEW_DISTANCE};
use crate::entity::player::Player;
use crate::network::connection::Connection;
use crate::network::packet::Encode;
use crate::registry::dimension_type::DIMENSION_TYPES;
use crate::registry::DIMENSION_TYPES_INDEX;
use crate::util::encode_position;
use crate::util::io::WriteExt;
use anyhow::anyhow;
use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct PlayLoginS2C {
    pub player: Arc<Mutex<Player>>,
}

impl Encode for PlayLoginS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> Result<()> {
        let (eid, dim, game_mode, previous_game_mode, death_location, portal_cooldown) = {
            let player = self.player.lock();
            let eid = player.entity.entity_id;
            let dim = player.entity.dimension;
            let game_mode = player.game_mode;
            let previous_game_mode = player.previous_game_mode;
            let death_location = player.death_location.clone();
            let portal_cooldown = player.entity.portal_cooldown;
            (
                eid,
                dim,
                game_mode,
                previous_game_mode,
                death_location,
                portal_cooldown,
            )
        };
        buf.write_i32(eid).await?;
        buf.write_bool(false).await?;
        buf.write_var_int(DIMENSION_TYPES.len() as i32).await?;
        for dimension_type in DIMENSION_TYPES_INDEX.iter() {
            buf.write_str(dimension_type).await?;
        }
        buf.write_var_int(*MAX_PLAYERS).await?;
        buf.write_var_int(*VIEW_DISTANCE).await?;
        buf.write_var_int(*SIMULATION_DISTANCE).await?;
        buf.write_bool(false).await?;
        buf.write_bool(true).await?;
        buf.write_bool(false).await?;
        buf.write_var_int(dim as i32).await?;
        buf.write_str(match DIMENSION_TYPES_INDEX.get(dim) {
            Some(index) => index,
            None => return Err(anyhow!("Dimension type not found")),
        })
        .await?;
        buf.write_i64(*HASHED_SEED).await?;
        buf.write_u8(game_mode).await?;
        buf.write_i8(previous_game_mode).await?;
        buf.write_bool(false).await?;
        buf.write_bool(false).await?;
        match &death_location {
            Some(death_location) => {
                buf.write_bool(true).await?;
                buf.write_str(&death_location.0).await?;
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
        buf.write_var_int(portal_cooldown).await?;
        buf.write_bool(false).await?;
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x2B
    }
}
