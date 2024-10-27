pub mod arc_channel;
pub mod io;
pub mod raw;

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
    let mut value = 0;
    for i in 0..5 {
        let byte = reader.read_u8().await?;
        value |= (byte as i32 & 0x7F) << (i * 7);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "VarInt too big"))
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

pub async fn write_str<W: AsyncWrite + Unpin>(writer: &mut W, value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    write_var_int(writer, bytes.len() as i32).await?;
    writer.write_all(bytes).await?;
    Ok(())
}
