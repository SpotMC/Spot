use crate::entity::player::Player;
use crate::entity::Entity::Player as EPlayer;
use crate::network::connection::Connection;
use crate::network::connection::State::Play;
use crate::network::packet::s2c::play_login_s2c::PlayLoginS2C;
use crate::WORLD;
use rand::random;
use std::io::Error;
use std::sync::mpsc::channel;
use tokio::io::AsyncRead;

pub(crate) async fn acknowledge_finish_configuration<R: AsyncRead + Unpin>(
    connection: &mut Connection<'_>,
    _data: R,
) -> Result<(), Error> {
    unsafe {
        let world = WORLD.get_mut();
        let wsp = world.get_world_spawn_point();
        let mut eid: i32 = random();
        while world.entities.get(eid).is_some() {
            eid = random();
        }
        let (tx, recv) = channel();
        let player = Player {
            health: 20.0,
            max_health: 20,
            dimension: wsp.0,
            entity_id: eid,
            game_mode: 0,
            previous_game_mode: -1,
            death_location: None,
            portal_cooldown: 0,
            pos: (wsp.1 as f64, wsp.2 as f64, wsp.3 as f64),
            velocity: (0.0, 0.0, 0.0),
            on_ground: false,
            yaw: 0.0,
            pitch: 0.0,
            tx,
        };
        connection.recv = Some(recv);
        connection
            .send_packet(&PlayLoginS2C { player: &player })
            .await?;
        world.entities.spawn(EPlayer(player), eid);
        connection.player = Some(eid);
    }
    connection.state = Play;
    Ok(())
}
