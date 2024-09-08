use crate::entity::player::Player;
use crate::network::connection::Connection;
use crate::network::connection::State::Play;
use crate::network::packet::s2c::play_login_s2c::PlayLoginS2C;
use crate::util::direct_pointer::DirectPointer;
use crate::WORLD;
use rand::random;
use std::io::Error;
use tokio::io::AsyncRead;
use tokio::sync::mpsc::unbounded_channel;

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
        let (tx, recv) = unbounded_channel();
        let mut player = Player::new(eid, wsp.0, tx, (wsp.1 as f64, wsp.2 as f64, wsp.3 as f64));
        connection.recv = Some(recv);
        connection.player = Some(DirectPointer(&mut player as *mut Player));
        world.entities.spawn(Box::from(player), eid);
        connection.player_eid = Some(eid);
        connection
            .send_packet(&PlayLoginS2C {
                player: world.entities.get(eid).unwrap().downcast_ref().unwrap(),
            })
            .await?;
    }
    connection.state = Play;
    Ok(())
}
