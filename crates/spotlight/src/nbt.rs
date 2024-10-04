use bytes::BufMut;
use dashmap::DashMap;

#[macro_export]
macro_rules! nbt_byte {
    ($data:expr) => {
        Box::from(NbtByte::from($data))
    };
}

#[macro_export]
macro_rules! nbt_short {
    ($data:expr) => {
        Box::from(NbtShort::from($data))
    };
}

#[macro_export]
macro_rules! nbt_int {
    ($data:expr) => {
        Box::from(NbtInt::from($data))
    };
}

#[macro_export]
macro_rules! nbt_long {
    ($data:expr) => {
        Box::from(NbtLong::from($data))
    };
}

#[macro_export]
macro_rules! nbt_float {
    ($data:expr) => {
        Box::from(NbtFloat::from($data))
    };
}

#[macro_export]
macro_rules! nbt_double {
    ($data:expr) => {
        Box::from(NbtDouble::from($data))
    };
}

#[macro_export]
macro_rules! nbt_str {
    ($data:expr) => {
        Box::from(NbtString::from($data))
    };
}

pub fn serde_nbt<R>(value: NbtCompound, buf: &mut R)
where
    R: BufMut,
{
    buf.put_u8(0x0a);
    value.serialize(buf);
}
pub struct NbtCompound {
    pub data: DashMap<String, Box<dyn NbtTag>>,
}
impl NbtTag for NbtCompound {
    fn get_type(&self) -> u8 {
        10
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        self.data.iter().for_each(|k| {
            buf.put_u8(k.value().get_type());
            let key = k.key().as_bytes();
            buf.put_u16(key.len() as u16);
            buf.put_slice(key);
            k.value().serialize(buf);
        });
        buf.put_u8(0);
    }
}

pub struct NbtList<T: NbtTag> {
    pub data: Vec<T>,
    pub tag_type: u8,
}
impl<T: NbtTag> NbtTag for NbtList<T> {
    fn get_type(&self) -> u8 {
        9
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_u8(self.tag_type);
        buf.put_i32(self.data.len() as i32);
        self.data.iter().for_each(|k| k.serialize(buf));
    }
}

pub struct NbtByteArray {
    pub data: Vec<u8>,
}

impl NbtTag for NbtByteArray {
    fn get_type(&self) -> u8 {
        7
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i32(self.data.len() as i32);
        buf.put_slice(&self.data);
    }
}

pub struct NbtIntArray {
    pub data: Vec<i32>,
}

impl NbtTag for NbtIntArray {
    fn get_type(&self) -> u8 {
        11
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i32(self.data.len() as i32);
        self.data.iter().for_each(|k| buf.put_i32(*k));
    }
}

pub struct NbtLongArray {
    pub data: Vec<i64>,
}

impl NbtTag for NbtLongArray {
    fn get_type(&self) -> u8 {
        12
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i32(self.data.len() as i32);
        self.data.iter().for_each(|k| buf.put_i64(*k));
    }
}

pub struct NbtString {
    pub data: String,
}
impl NbtTag for NbtString {
    fn get_type(&self) -> u8 {
        8
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        let data = self.data.as_bytes();
        buf.put_u16(data.len() as u16);
        buf.put_slice(data);
    }
}

impl From<String> for NbtString {
    fn from(value: String) -> Self {
        NbtString { data: value }
    }
}

impl From<&str> for NbtString {
    fn from(value: &str) -> Self {
        NbtString {
            data: value.to_string(),
        }
    }
}

impl From<&String> for NbtString {
    fn from(value: &String) -> Self {
        NbtString {
            data: value.to_string(),
        }
    }
}

pub struct NbtByte {
    pub data: i8,
}

impl NbtTag for NbtByte {
    fn get_type(&self) -> u8 {
        1
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i8(self.data);
    }
}

impl From<i8> for NbtByte {
    fn from(value: i8) -> Self {
        NbtByte { data: value }
    }
}

pub struct NbtShort {
    pub data: i16,
}

impl NbtTag for NbtShort {
    fn get_type(&self) -> u8 {
        2
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i16(self.data);
    }
}

impl From<i16> for NbtShort {
    fn from(value: i16) -> Self {
        NbtShort { data: value }
    }
}

pub struct NbtInt {
    pub data: i32,
}

impl NbtTag for NbtInt {
    fn get_type(&self) -> u8 {
        3
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i32(self.data);
    }
}

impl From<i32> for NbtInt {
    fn from(value: i32) -> Self {
        NbtInt { data: value }
    }
}

pub struct NbtLong {
    pub data: i64,
}

impl NbtTag for NbtLong {
    fn get_type(&self) -> u8 {
        4
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_i64(self.data);
    }
}

impl From<i64> for NbtLong {
    fn from(value: i64) -> Self {
        NbtLong { data: value }
    }
}

pub struct NbtFloat {
    pub data: f32,
}

impl NbtTag for NbtFloat {
    fn get_type(&self) -> u8 {
        5
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_f32(self.data);
    }
}

impl From<f32> for NbtFloat {
    fn from(value: f32) -> Self {
        NbtFloat { data: value }
    }
}

pub struct NbtDouble {
    pub data: f64,
}

impl From<f64> for NbtDouble {
    fn from(value: f64) -> Self {
        NbtDouble { data: value }
    }
}

impl NbtTag for NbtDouble {
    fn get_type(&self) -> u8 {
        6
    }

    fn serialize(&self, buf: &mut dyn BufMut) {
        buf.put_f64(self.data);
    }
}

pub trait NbtTag {
    fn get_type(&self) -> u8;
    fn serialize(&self, buf: &mut dyn BufMut);
}
