use crate::network::connection::Connection;
use crate::network::packet::Encode;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct GameEventS2C {
    pub event: GameEvent,
    pub value: f32,
}

impl Encode for GameEventS2C {
    async fn encode<W: AsyncWrite + Unpin>(
        &self,
        _connection: &mut Connection<'_>,
        buf: &mut W,
    ) -> anyhow::Result<()> {
        buf.write_u8(self.event as u8).await?;
        buf.write_f32(self.value).await?;
        Ok(())
    }

    fn get_id(&self) -> i32 {
        0x22
    }
}

impl GameEventS2C {
    pub fn new(event: GameEvent, value: f32) -> GameEventS2C {
        GameEventS2C { event, value }
    }
    pub fn empty(event: GameEvent) -> GameEventS2C {
        GameEventS2C { event, value: 0.0 }
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum GameEvent {
    NoRespawnBlockAvailable = 0,
    BeginRaining = 1,
    EndRaining = 2,
    ChangeGameMode = 3,
    WinGame = 4,
    DemoEvent = 5,
    ArrowHitPlayer = 6,
    RainLevelChange = 7,
    ThunderLevelChange = 8,
    PlayPufferfishStingSound = 9,
    PlayElderGuardianMobAppearance = 10,
    EnableRespawnScreen = 11,
    LimitedCrafting = 12,
    StartWaitingForLevelChunks = 13,
}
