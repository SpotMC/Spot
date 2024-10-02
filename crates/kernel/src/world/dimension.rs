use crate::registry::dimension_type::DimensionType;
use crate::util::to_dim_xz;
use crate::world::chunk::Chunk;
use dashmap::DashMap;
use std::sync::{Arc, Weak};

pub struct Dimension {
    pub(crate) dim_idx: u32,
    pub dimension_type: DimensionType,
    pub dimension_name: String,
    pub chunks: DashMap<u64, Weak<Chunk>>,
}
impl Dimension {
    pub fn new(dimension_type: DimensionType, dimension_name: String, dim_idx: u32) -> Dimension {
        Dimension {
            dimension_type,
            dimension_name,
            chunks: DashMap::with_capacity(512),
            dim_idx,
        }
    }
    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Arc<Chunk> {
        let key = to_dim_xz(chunk_x, chunk_z);
        let tv = self.chunks.get_mut(&key);
        if tv.is_some() {
            let tvv = tv.unwrap().value_mut().upgrade();
            if let Some(ret) = tvv {
                return ret;
            }
        }
        let chunk = Arc::new(self.create_new_chunk(chunk_x, chunk_z));
        self.insert_new_chunk(chunk_x, chunk_z, &chunk);
        chunk
    }
    fn create_new_chunk(&self, x: i32, z: i32) -> Chunk {
        // TODO
        Chunk::new(self, to_dim_xz(x, z))
    }
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let min_y = self.dimension_type.min_y;
        let chunk = self.get_chunk(chunk_x, chunk_z);
        chunk.get_block(x - chunk_x * 16, y - min_y, z - chunk_z * 16)
    }
    pub fn set_block(&self, x: i32, y: i32, z: i32, block: u32) -> Arc<Chunk> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let min_y = self.dimension_type.min_y;
        let chunk = self.get_chunk(chunk_x, chunk_z);
        chunk.set_block(x - chunk_x * 16, y - min_y, z - chunk_z * 16, block);
        chunk
    }
    fn insert_new_chunk(&self, x: i32, z: i32, chunk: &Arc<Chunk>) {
        self.chunks.insert(to_dim_xz(x, z), Arc::downgrade(chunk));
    }
}
