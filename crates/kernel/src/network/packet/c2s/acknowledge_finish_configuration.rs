use crate::entity::player::Player;
use crate::entity::Entity;
use crate::network::connection::Connection;
use crate::network::connection::State::Play;
use crate::network::packet::s2c::play_login_s2c::PlayLoginS2C;
use crate::WORLD;
use anyhow::Result;
use async_trait::async_trait;
use fastrand::i32;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;

pub struct AcknowledgeFinishConfiguration;
#[async_trait]
impl crate::network::packet::Decode for AcknowledgeFinishConfiguration {
    async fn decode(&self, connection: &mut Connection<'_>, _data: Vec<u8>) -> Result<()> {
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
}
