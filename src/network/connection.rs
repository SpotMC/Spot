use crate::network::connection::State::{Handshake, Login};
use crate::network::packet::c2s::config_client_info_c2s::client_information;
use crate::network::packet::c2s::config_known_packs_c2s::known_packs;
use crate::network::packet::c2s::login_acknowledged_c2s::login_acknowledged;
use crate::network::packet::c2s::login_start_c2s::login_start;
use crate::network::packet::Encode;
use crate::util::{read_str, read_var_int, write_var_int};
use crate::{read_str, read_var_int, PROTOCOL_VERSION};
use bytes::BytesMut;
use std::io::{Error, ErrorKind};
use std::pin::{pin, Pin};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::task::yield_now;

macro_rules! decoders {
    ($connection:expr;$packet:expr; $($x:expr => $y:ident)*) => {
        match $packet.id {
            $($x=>{$y(&mut $connection, &mut $packet.data.as_ref()).await?})*
            _ => {}
        }
    };
}

pub(crate) async fn read_socket(socket: &mut TcpStream) -> Result<(), Error> {
    let mut connection = Connection::new(socket);
    loop {
        let packet = connection.read_packet().await?;
        yield_now().await;
        match connection.state {
            Handshake => {
                let mut data: Pin<&mut Box<&[u8]>> = pin!(Box::new(packet.data.as_ref()));
                let protocol_version = read_var_int!(data);
                if protocol_version != PROTOCOL_VERSION {
                    return Err(Error::new(
                        ErrorKind::Unsupported,
                        format!("Invalid protocol version: {protocol_version}"),
                    ));
                }
                let _server_addr = read_str!(data);
                let _server_port = data.read_u16().await?;
                let next_state = read_var_int!(data);
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
                    0x07 => known_packs
                )
            }
            State::Play => {}
        }
        yield_now().await;
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
        }
    }
    pub(crate) async fn send_packet<D: Encode>(&mut self, data: &D) -> Result<(), Error> {
        let mut buf = Vec::with_capacity(1024);
        write_var_int(&mut buf, data.get_id()).await?;
        data.encode(self, &mut buf).await?;
        write_var_int(self.stream, buf.len() as i32).await?;
        self.stream.write_all(&buf).await?;
        Ok(())
    }
    async fn read_packet(&mut self) -> Result<Packet, Error> {
        self.stream.readable().await?;
        let length = read_var_int!(self.stream);
        let mut buf = BytesMut::with_capacity(length as usize);
        self.stream.read_buf(&mut buf).await?;
        let id = read_var_int!(buf.as_ref());
        Ok(Packet { id, data: buf })
    }
}

pub struct Packet {
    pub id: i32,
    pub data: BytesMut,
}
pub enum State {
    Handshake,
    Login,
    Configuration,
    Play,
}
