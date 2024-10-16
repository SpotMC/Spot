use crate::entity::player::Player;
use crate::entity::Entity;
use crate::network::connection::Connection;
use crate::network::connection::State::Play;
use crate::network::packet::s2c::play_login_s2c::PlayLoginS2C;
use crate::WORLD;
use anyhow::Result;
use fastrand::i32;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::io::AsyncRead;
use tokio::sync::mpsc::unbounded_channel;

pub(crate) async fn acknowledge_finish_configuration<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    _data: R,
) -> Result<()> {
    let mut eid;
    let arc: Arc<Mutex<Player>>;
    {
        let wsp = WORLD.get_world_spawn_point();
        eid = i32(i32::MIN..i32::MAX);
        while WORLD.entities.get_mut(eid).is_some() {
            eid = i32(i32::MIN..i32::MAX);
        }
        let (tx, recv) = unbounded_channel();
        let player = Player::new(eid, wsp.0, tx, (wsp.1 as f64, wsp.2 as f64, wsp.3 as f64));
        connection.recv = Some(recv);
        arc = Arc::new(Mutex::new(player));
        connection.player = Some(arc.clone());
        WORLD
            .entities
            .spawn(&(arc.clone() as Arc<Mutex<dyn Entity>>));
    }
    connection.player_eid = Some(eid);
    connection
        .send_packet(&PlayLoginS2C { player: arc })
        .await?;
    connection.state = Play;
    Ok(())
}
