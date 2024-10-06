use crate::config::WORLDGEN_IMPLEMENTATION;
use crate::registry::dimension_type::DimensionType;
use crate::util::to_dim_xz;
use crate::world::chunk::Chunk;
use crate::world::gen::{Worldgen, IMPLEMENTS};
use dashmap::DashMap;
use std::sync::{Arc, Weak};

pub struct Dimension {
    pub(crate) dim_idx: u32,
    pub dimension_type: DimensionType,
    pub dimension_name: String,
    pub chunks: DashMap<u64, Weak<Chunk>>,
    worldgen: &'static dyn Worldgen,
}
impl Dimension {
    /// Creates a new dimension object.
    ///
    /// # Arguments
    /// - `dimension_type`: The type of dimension, defining its basic characteristics.
    /// - `dimension_name`: The name of the dimension, used to identify it.
    /// - `Dim_idx`: The index of the dimension, used for unique identification among multiple dimensions.
    ///
    /// # Returns
    /// -
    /// Returns a `Dimension`
    /// object containing all information and configurations related to the dimension.
    ///
    /// # Description
    /// This function initializes a dimension object based on the given dimension type,
    /// name, and index.
    /// It also configures a world generator for the dimension object by looking up implementation details.
    /// The world generator is responsible
    /// for generating the logic and rules of the dimension world based on the configuration of the dimension object.
    pub fn new(dimension_type: DimensionType, dimension_name: String, dim_idx: u32) -> Dimension {
        Dimension {
            dimension_type,
            dimension_name,
            chunks: DashMap::with_capacity(512),
            dim_idx,
            worldgen: &**IMPLEMENTS
                .get(&WORLDGEN_IMPLEMENTATION.to_string())
                .unwrap(),
        }
    }

    /// Get the chunk at the specified coordinates.
    ///
    /// If the chunk already exists, it returns the existing chunk,
    /// otherwise, it creates a new chunk and returns it.
    ///
    /// # Parameters
    /// - `chunk_x`: The X coordinate of the chunk.
    /// - `chunk_z`: The Z coordinate of the chunk.
    ///
    /// # Returns
    /// Returns an `Arc`-wrapped `Chunk` object representing the chunk at the specified location.

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
        let chunk = Chunk::new(self, to_dim_xz(x, z));
        self.worldgen.gen(chunk)
    }
    /// Get the block ID at the specified coordinates (x, y, z).
    ///
    /// # Parameters
    /// - `x`: The x coordinate of the block.
    /// - `y`: The y coordinate of the block.
    /// - `Z`: The z coordinate of the block.
    ///
    /// # Returns
    /// - `Option<u32>`:
    ///   Returns Some(block ID) if there is a block at the given coordinates,
    ///   otherwise returns None.
    ///
    /// # Description
    /// This function first calculates the chunk coordinates based on the x and z coordinates,
    /// then retrieves the chunk object.
    /// Next, it uses the local coordinates relative to the chunk, along with the y
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let min_y = self.dimension_type.min_y;
        let chunk = self.get_chunk(chunk_x, chunk_z);
        chunk.get_block(x - chunk_x * 16, y - min_y, z - chunk_z * 16)
    }
    /// Sets the block type at the given coordinates.
    ///
    /// # Parameters
    /// - `x`: The X coordinate of the block.
    /// - `y`: The Y coordinate of the block.
    /// - `z`: The Z coordinate of the block.
    /// - `block`: The type of block to set.
    ///
    /// # Returns
    /// Returns the chunk containing the modified block.
    ///
    /// # Description
    /// This function determines the chunk based on the given block coordinates
    /// and sets the new block type within that chunk.
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
