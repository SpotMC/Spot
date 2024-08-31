use std::io::{Error, ErrorKind};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[inline]
pub fn to_chunk_yzx(x: i32, y: i32, z: i32) -> usize {
    (y << 8 | z << 4 | x) as usize
}

#[inline]
pub fn to_dim_xz(x: i32, z: i32) -> u64 {
    (x as u64) << 32 | (z as u64)
}
pub fn encode_position(x: i32, y: i32, z: i32) -> u64 {
    ((x as u64 & 0x3FFFFFF) << 38) | ((z as u64 & 0x3FFFFFF) << 12) | (y as u64 & 0xFFF)
}

pub async fn read_var_int<R: AsyncRead + Unpin>(reader: &mut R) -> Result<i32, Error> {
    let mut val = 0;
    for i in 0..5 {
        let byte = reader.read_u8().await?;
        val |= (i32::from(byte) & 0b01111111) << (i * 7);
        if byte & 0b10000000 == 0 {
            return Ok(val);
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "VarInt too big"))
}

#[macro_export]
macro_rules! read_var_int {
    ($reader:expr) => {
        read_var_int(&mut $reader).await?
    };
}
pub async fn write_var_int<W: AsyncWrite + Unpin>(writer: &mut W, value: i32) -> Result<(), Error> {
    let mut value = value as u32;
    loop {
        if (value & !0x7F) == 0 {
            writer.write_u8(value as u8).await?;
            break;
        }
        writer.write_u8(((value & 0x7F) | 0x80) as u8).await?;
        value >>= 7;
    }
    Ok(())
}

pub async fn read_str<R: AsyncRead + Unpin>(reader: &mut R) -> Result<String, Error> {
    let len = read_var_int(reader).await?;
    let mut buf = Vec::with_capacity(len as usize);
    reader.read_buf(&mut buf).await?;
    match String::from_utf8(buf) {
        Ok(value) => Ok(value),
        Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
    }
}

#[macro_export]
macro_rules! read_str {
    ($reader:expr) => {
        read_str(&mut $reader).await?
    };
}

pub async fn write_str<W: AsyncWrite + Unpin>(writer: &mut W, value: &str) -> Result<(), Error> {
    write_var_int(writer, value.len() as i32).await?;
    writer.write_all(value.as_bytes()).await?;
    Ok(())
}

#[macro_export]
macro_rules! write_bool {
    ($writer:expr, $bool:expr) => {
        $writer.write_u8(if $bool { 1 } else { 0 }).await?
    };
}
#[macro_export]
macro_rules! read_bool {
    ($reader:expr) => {
        $reader.read_u8().await? != 0
    };
}

#[macro_export]
macro_rules! block_on {
    ($block:block) => {
        match tokio::runtime::Builder::new_current_thread().build() {
            Ok(value) => value.block_on(async { $block }),
            Err(err) => Err(Self::Error::new(err.to_string())),
        }
    };
}

#[macro_export]
macro_rules! unwrap {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(err) => return Err(Self::Error::new(err.to_string())),
        }
    };
}
