use crate::registry::dimension_type::DimensionType;
use crate::util::to_dim_xz;
use crate::world::chunk::Chunk;
use nohash_hasher::{BuildNoHashHasher, IntMap};

pub struct Dimension {
    pub dimension_type: DimensionType,
    pub dimension_name: String,
    pub chunks: IntMap<u64, Chunk>,
}
impl Dimension {
    pub fn new(dimension_type: DimensionType, dimension_name: String) -> Dimension {
        Dimension {
            dimension_type,
            dimension_name,
            chunks: IntMap::with_capacity_and_hasher(256, BuildNoHashHasher::default()),
        }
    }
    pub fn get_chunk(&mut self, x: i32, z: i32) -> Option<&mut Chunk> {
        match self.chunks.get_mut(&to_dim_xz(x, z)) {
            Some(chunk) => Some(chunk),
            None => {
                todo!()
            }
        }
    }
    fn create_new_chunk(&mut self, _x: i32, _z: i32) -> Chunk {
        todo!()
    }
    pub fn get_block(&mut self, x: i32, y: i32, z: i32) -> Option<u32> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let chunk = match self.get_chunk(chunk_x, chunk_z) {
            Some(chunk) => chunk,
            None => return None,
        };
        chunk.get_block(x - chunk_x * 16, y, z - chunk_z * 16)
    }
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let chunk = match self.get_chunk(chunk_x, chunk_z) {
            Some(chunk) => chunk,
            None => {
                let mut chunk = self.create_new_chunk(chunk_x, chunk_z);
                chunk.set_block(x - chunk_x * 16, y, z - chunk_z * 16, block);
                self.insert_new_chunk(chunk_x, chunk_z, chunk);
                self.get_chunk(chunk_x, chunk_z).unwrap()
            }
        };
        chunk.set_block(x - chunk_x * 16, y, z - chunk_z * 16, block);
    }
    fn insert_new_chunk(&mut self, x: i32, z: i32, chunk: Chunk) {
        self.chunks.insert(to_dim_xz(x, z), chunk);
    }
}
