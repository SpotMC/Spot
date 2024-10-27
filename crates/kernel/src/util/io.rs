use crate::util::{read_str, read_var_int, write_str, write_var_int};
use bit_set::BitSet;
use std::io::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[allow(async_fn_in_trait)]
pub trait ReadExt: AsyncRead + Unpin {
    async fn read_var_int(&mut self) -> Result<i32, Error>;
    async fn read_str(&mut self) -> Result<String, Error>;
    async fn read_bool(&mut self) -> Result<bool, Error>;
}

impl<T: AsyncRead + Unpin> ReadExt for T {
    async fn read_var_int(&mut self) -> Result<i32, Error> {
        read_var_int(self).await
    }
    async fn read_str(&mut self) -> Result<String, Error> {
        read_str(self).await
    }
    async fn read_bool(&mut self) -> Result<bool, Error> {
        if self.read_u8().await? == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
#[allow(async_fn_in_trait)]
pub trait WriteExt: AsyncWrite + Unpin {
    async fn write_var_int(&mut self, value: i32) -> Result<(), Error>;
    async fn write_str(&mut self, value: &str) -> Result<(), Error>;
    async fn write_bool(&mut self, value: bool) -> Result<(), Error>;
    async fn write_bitset(&mut self, value: &BitSet) -> Result<(), Error>;
}

impl<T: AsyncWrite + Unpin> WriteExt for T {
    async fn write_var_int(&mut self, value: i32) -> Result<(), Error> {
        write_var_int(self, value).await
    }
    async fn write_str(&mut self, value: &str) -> Result<(), Error> {
        write_str(self, value).await
    }
    async fn write_bool(&mut self, value: bool) -> Result<(), Error> {
        if value {
            self.write_u8(1).await?;
        } else {
            self.write_u8(0).await?;
        }
        Ok(())
    }

    async fn write_bitset(&mut self, value: &BitSet) -> Result<(), Error> {
        let array = value.get_ref().storage();
        let mut vec = Vec::with_capacity(array.len() / 2);
        for i in (0..array.len()).step_by(2) {
            let mut entry: u64 = array[i] as u64;
            if i + 1 < array.len() {
                entry |= (array[i + 1] as u64) << 32;
            } else {
                break;
            }
            vec.push(entry);
        }
        write_var_int(self, vec.len() as i32).await?;
        for entry in vec {
            self.write_u64(entry).await?;
        }
        Ok(())
    }
}
