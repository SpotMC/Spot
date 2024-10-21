use crate::entity::player::Player;
use crate::entity::Entity;
use crate::network::connection::Connection;
use crate::network::packet::s2c::game_event::{GameEvent, GameEventS2C};
use crate::network::packet::s2c::play_login::PlayLoginS2C;
use crate::network::packet::s2c::set_center_chunk::SetCenterChunkS2C;
use crate::network::packet::s2c::synchronize_player_position::SynchronizePlayerPositionS2C;
use crate::WORLD;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;
use uuid::Uuid;

pub(crate) async fn player_join(connection: &mut Connection<'_>) -> anyhow::Result<()> {
    let eid;
    let arc: Arc<Mutex<Player>>;
    let (chunk_x, chunk_z);
    {
        let wsp = WORLD.get_world_spawn_point();
        eid = WORLD.entities.generate_eid();
        let (tx, recv) = unbounded_channel();
        let player = Player::new(
            eid,
            wsp.0,
            tx,
            (wsp.1 as f64, wsp.2 as f64, wsp.3 as f64),
            Uuid::from_u128(connection.uuid.unwrap()),
        );
        connection.recv = Some(recv);
        chunk_x = player.entity.pos.0 as i32 >> 4;
        chunk_z = player.entity.pos.2 as i32 >> 4;
        arc = Arc::new(Mutex::new(player));
        connection.player = Some(arc.clone());
        WORLD
            .entities
            .spawn(&(arc.clone() as Arc<Mutex<dyn Entity>>));
    }
    connection.player_eid = Some(eid);
    connection.send_packet(&PlayLoginS2C).await?;
    connection
        .send_packet(&GameEventS2C::empty(GameEvent::StartWaitingForLevelChunks))
        .await?;
    connection
        .send_packet(&SynchronizePlayerPositionS2C)
        .await?;
    connection
        .send_packet(&SetCenterChunkS2C { chunk_x, chunk_z })
        .await?;
    Ok(())
}
