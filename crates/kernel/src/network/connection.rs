use crate::entity::player::{Player, PlayerUpdate};
use crate::network::connection::State::{Handshake, Login};
use crate::network::packet::*;
use crate::util::io::{ReadExt, WriteExt};
use crate::PROTOCOL_VERSION;
use anyhow::anyhow;
use anyhow::Result;
use parking_lot::Mutex;
use std::pin::{pin, Pin};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::yield_now;

pub(crate) async fn read_socket(socket: &mut TcpStream) -> Result<()> {
    socket.set_nodelay(true)?;
    let mut connection = Connection::new(socket);
    loop {
        let packet = connection.read_packet().await?;
        match connection.state {
            Handshake => {
                let mut data: Pin<&mut Box<&[u8]>> = pin!(Box::new(&*packet.data));
                data.read_var_int().await?;
                let protocol_version = data.read_var_int().await?;
                if protocol_version != PROTOCOL_VERSION {
                    return Err(anyhow!("Invalid protocol version {}", protocol_version));
                }
                let _server_addr = data.read_str().await?;
                let _server_port = data.read_u16().await?;
                let next_state = data.read_var_int().await?;
                if next_state != 2 {
                    return Ok(());
                }
                connection.state = Login
            }
            Login => {
                if let Some(decoder) = LOGIN_DECODERS.load().get(&packet.id) {
                    decoder.decode(&mut connection, packet.data).await?;
                }
            }
            State::Configuration => {
                if let Some(decoder) = CONFIGURATION_DECODERS.load().get(&packet.id) {
                    decoder.decode(&mut connection, packet.data).await?;
                }
            }
            State::Play => {
                if let Some(decoder) = PLAY_DECODERS.load().get(&packet.id) {
                    decoder.decode(&mut connection, packet.data).await?;
                }
            }
        }
        yield_now().await;
        if let Some(recv) = &mut connection.recv {
            while let Ok(_update) = &recv.try_recv() {
                // TODO
            }
        }
    }
}
pub struct Connection<'a> {
    pub stream: &'a mut TcpStream,
    pub state: State,
    pub username: Option<String>,
    pub uuid: Option<u128>,
    pub locale: Option<String>,
    pub view_distance: Option<i8>,
    pub chat_mode: Option<ChatMode>,
    pub chat_colors: Option<bool>,
    pub skin_parts: Option<u8>,
    pub main_hand: Option<MainHand>,
    pub enable_text_filtering: Option<bool>,
    pub allow_server_listings: Option<bool>,
    pub player_eid: Option<i32>,
    pub player: Option<Arc<Mutex<Player>>>,
    pub recv: Option<UnboundedReceiver<PlayerUpdate>>,
}

pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}
pub enum MainHand {
    Left,
    Right,
}
impl Connection<'_> {
    pub(crate) fn new(stream: &mut TcpStream) -> Connection<'_> {
        Connection {
            stream,
            state: Handshake,
            username: None,
            uuid: None,
            locale: None,
            view_distance: None,
            chat_mode: None,
            chat_colors: None,
            skin_parts: None,
            main_hand: None,
            enable_text_filtering: None,
            allow_server_listings: None,
            player: None,
            player_eid: None,
            recv: None,
        }
    }
    pub async fn send_packet<D: Encode>(&mut self, data: &D) -> Result<()> {
        let mut buf = Vec::new();
        buf.write_var_int(data.get_id()).await?;
        data.encode(self, &mut buf).await?;
        self.stream.write_var_int(buf.len() as i32).await?;
        self.stream.write_all(&buf).await?;
        Ok(())
    }
    async fn read_packet(&mut self) -> Result<Packet> {
        self.stream.readable().await?;
        let length = self.stream.read_var_int().await?;
        let mut buf = Vec::with_capacity(length as usize);
        self.stream.read_buf(&mut buf).await?;
        let id = buf.as_slice().read_var_int().await?;
        Ok(Packet { id, data: buf })
    }
}

pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}
pub enum State {
    Handshake,
    Login,
    Configuration,
    Play,
}
