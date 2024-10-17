use crate::entity::player::{Player, PlayerUpdate};
use crate::network::connection::State::{Handshake, Login};
use crate::network::packet::c2s::acknowledge_finish_configuration::acknowledge_finish_configuration;
use crate::network::packet::c2s::client_info::client_information;
use crate::network::packet::c2s::known_packs_c2s::known_packs;
use crate::network::packet::c2s::login_acknowledged::login_acknowledged;
use crate::network::packet::c2s::login_start::login_start;
use crate::network::packet::Encode;
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

macro_rules! decoders {
    ($connection:expr;$packet:expr; $($x:expr => $y:ident)*) => {
        match $packet.id {
            $($x=>{
                let mut data = &*$packet.data;
                data.read_var_int().await?;
                $y(&mut $connection, &mut data).await?
            })*
            _ => {}
        }
    };
}

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
                decoders!(connection;packet;
                    0x00 => login_start
                    0x03 => login_acknowledged
                )
            }
            State::Configuration => {
                decoders!(connection;packet;
                    0x00 => client_information
                    0x03 => acknowledge_finish_configuration
                    0x07 => known_packs
                )
            }
            State::Play => {}
        }
        yield_now().await;
        if let Some(recv) = &mut connection.recv {
            while let Ok(_update) = &recv.try_recv() {
                // TODO
            }
        }
    }
}
pub(crate) struct Connection<'a> {
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

pub(crate) enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}
pub(crate) enum MainHand {
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
    pub(crate) async fn send_packet<D: Encode>(&mut self, data: &D) -> Result<()> {
        let mut buf = Vec::with_capacity(1024);
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
