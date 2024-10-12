use crate::util::io::WriteExt;
use crate::world::dimension::Dimension;
use crate::WORLD;
use hashbrown::HashMap;
use parking_lot::{Mutex, MutexGuard};
use std::sync::atomic::{AtomicI16, Ordering};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct Chunk {
    pub(crate) data: Vec<Section>,
    pub(crate) pos: u64,
    pub(crate) height: i32,
    pub(crate) idx: u32,
}

impl Chunk {
    /// Creates a new Chunk instance.
    ///
    /// # Parameters
    /// - `dimension`:
    ///   A reference to dimension information
    ///   used to fetch dimension-specific data such as height.
    /// - `Pos`:
    ///   The position of the Chunk, represented as an unsigned 64-bit integer.
    ///   The First 32 bits represent the X coordinate, the next 32 bits represent the Z coordinate.
    ///
    /// # Returns
    /// Returns a Chunk instance with initialized data and properties.
    pub fn new(dimension: &Dimension, pos: u64) -> Chunk {
        let height = dimension.dimension_type.height;
        let size = (height / 16) as usize;
        let mut data = Vec::with_capacity(size);
        while data.len() < size {
            data.push(Section::new());
        }
        Chunk {
            data,
            height,
            idx: dimension.dim_idx,
            pos,
        }
    }
    /// Get the block information at the specified coordinates (x, y, z).
    ///
    /// This function queries the spatial data structure
    /// represented by the current object based on the given 3D coordinates.
    /// If the coordinate position is valid and the block information exists,
    /// it returns the block ID wrapped in Some;
    /// otherwise, it returns None.
    ///
    /// # Arguments
    /// - `x` - The x-coordinate of the block.
    /// - `y` - The y-coordinate of the block.
    /// - `z` - The z-coordinate of the block.
    ///
    /// # Returns
    /// - `Option<u32>` - The block ID if found, otherwise None.
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let idx = check_pos(x, y, z, self.height)?;
        let section = self.data.get(idx)?;
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(unsafe { section.get_state_unchecked(x as u32, sy, z as u32) })
    }
    /// Sets a block type at a specific 3D coordinate position.
    ///
    /// # Parameters
    /// - `x`: The X coordinate of the block.
    /// - `y`: The Y coordinate of the block.
    /// - `z`: The Z coordinate of the block.
    /// - `block`: The type of block to set.
    ///
    /// This function first validates if the given coordinates are within the valid range
    /// (based on the height).
    /// If the position is invalid, it returns immediately.
    /// If the position is valid, it retrieves the corresponding section (`section`) for that position.
    /// If the section does not exist, it returns immediately.
    /// Finally, it sets the specified block type within the determined section.
    pub fn set_block(&self, x: i32, y: i32, z: i32, block: u32) {
        let idx = match check_pos(x, y, z, self.height) {
            Some(s) => s,
            None => return,
        };
        let section = match self.data.get(idx) {
            Some(s) => s,
            None => return,
        };
        let sy = ((y as usize) - (16 * idx)) as u32;
        section.set_state(x as u32, sy, z as u32, block)
    }
    /// Get the section data at the specified index
    ///
    /// # Parameters
    /// - `y`: The index of the section, represented as an usize
    ///
    /// # Returns
    /// - `Option<SectionDataGuard>`:
    ///   Returns a Some wrapping a SectionDataGuard object
    ///   if the section data is successfully retrieved,
    ///   otherwise returns None
    ///
    /// # Notes
    /// This method retrieves the section data based on the provided section
    pub fn get_section(&self, y: usize) -> Option<SectionDataGuard> {
        Some(self.data.get(y / 16)?.get_data_guard())
    }
    /// Get a guard object that provides pre-locked access to the chunk data.
    pub fn get_guard(&self) -> ChunkGuard<'_> {
        let sections = self.data.len();
        let mut data = Vec::with_capacity(sections);
        for i in 0..sections {
            data.push(self.data.get(i).unwrap().get_data_guard());
        }
        ChunkGuard {
            data,
            height: self.height,
        }
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        WORLD
            .read()
            .dimensions
            .get(self.idx as usize)
            .unwrap()
            .chunks
            .remove(&self.pos);
    }
}

pub struct ChunkGuard<'a> {
    data: Vec<SectionDataGuard<'a>>,
    height: i32,
}

impl ChunkGuard<'_> {
    /// Retrieves the block information at the specified coordinates (x, y, z).
    ///
    /// This function queries the spatial data structure represented by the current object and returns
    /// the identifier of the block at the given position.
    /// If the coordinate position is valid and the
    /// block information exists, it returns Some wrapping the block identifier;
    /// otherwise, it returns None.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate, representing the horizontal position.
    /// - `Y`: The y-coordinate, representing the vertical position.
    /// - `Z`: The z-coordinate, representing the depth position.
    ///
    /// # Returns
    /// - `Option<u32>`:
    ///   Some wrapping the block identifier
    ///   if the query is successful or the coordinate is invalid.
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let idx = check_pos(x, y, z, self.height)?;
        let section = self.data.get(idx)?;
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(unsafe { section.get_state_unchecked(x as u32, sy, z as u32) })
    }
    /// Sets the block type at a specified position.
    ///
    /// # Parameters
    /// - `x`: The X coordinate of the block.
    /// - `y`: The Y coordinate of the block.
    /// - `z`: The Z coordinate of the block.
    /// - `block`: The block type to set.
    ///
    /// # Description
    /// This function sets the block type at the given coordinates in the world.
    /// It first validates the coordinates and then finds the corresponding chunk.
    /// If the coordinates are out of bounds or the chunk is not accessible, the operation is ignored.
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) {
        let idx = match check_pos(x, y, z, self.height) {
            Some(s) => s,
            None => return,
        };
        let section = match self.data.get_mut(idx) {
            Some(s) => s,
            None => return,
        };
        let sy = ((y as usize) - (16 * idx)) as u32;
        section.set_state(x as u32, sy, z as u32, block)
    }
}

#[inline]
fn check_pos(x: i32, y: i32, z: i32, height: i32) -> Option<usize> {
    if y < 0 || y >= height {
        return None;
    }
    if !(0..16).contains(&x) || !(0..16).contains(&z) {
        return None;
    }
    Some(y as usize / 16)
}

pub struct Section {
    pub data: Mutex<[u32; 4096]>,
    pub block_count: AtomicI16,
}

impl Section {
    pub fn new() -> Section {
        Section {
            data: Mutex::new([0; 4096]),
            block_count: AtomicI16::new(0),
        }
    }

    pub fn get_state(&self, x: u32, y: u32, z: u32) -> u32 {
        let index = (x + 16 * (y + 16 * z)) as usize;
        self.data.lock()[index]
    }
    /// Gets the state at the given coordinates (x, y, z) unsafely.
    ///
    /// # Safety
    /// - The caller must ensure that the coordinates (x, y, z)
    ///   do not exceed the bounds of the internal data array.
    /// - This function does not check the validity of the coordinates,
    ///   and incorrect coordinates may lead to undefined behavior.
    ///
    /// # Parameters
    /// - `x`: The x coordinate, ranging from 0 to 15.
    /// - `y`: The y coordinate, ranging from 0 to 15.
    /// - `Z`: The z coordinate, ranging from 0 to 15.
    ///
    /// # Returns
    /// - Returns the state value at the given coordinates.
    ///
    /// # Locking
    /// - This function locks the internal data to ensure data consistency.
    #[inline]
    pub unsafe fn get_state_unchecked(&self, x: u32, y: u32, z: u32) -> u32 {
        let index = (x + 16 * (y + 16 * z)) as usize;
        *self.data.lock().get_unchecked(index)
    }

    #[inline]
    pub fn set_state(&self, x: u32, y: u32, z: u32, state: u32) {
        let index = (x + 16 * (y + 16 * z)) as usize;
        let mut data = self.data.lock();
        if state != 0 && unsafe { data.get_unchecked(index) == &0 } {
            self.block_count.fetch_add(1, Ordering::Relaxed);
        } else if state == 0 && unsafe { data.get_unchecked(index) != &0 } {
            self.block_count.fetch_sub(1, Ordering::Relaxed);
        }
        data[index] = state;
    }

    #[inline]
    pub fn get_data_guard(&self) -> SectionDataGuard {
        SectionDataGuard {
            data: self.data.lock(),
            count: &self.block_count,
        }
    }

    /// Serializes the data and writes it to an asynchronous writer.
    ///
    /// This function is responsible for converting the internally stored data into a compact format
    /// and writing it into the provided `buffer`,
    /// which implements the asynchronous writing interface.
    /// The serialization process entails constructing a palette, a mapping of IDs to data elements,
    /// alongside a sequence describing how each element is encoded within the palette using bits.
    ///
    /// # Arguments
    /// -
    /// `buffer`: An object implementing the asynchronous writing interface with the `Unpin` trait,
    /// to which the serialized data will be written.
    ///
    /// # Returns
    /// Returns an `anyhow::Result<()>`, indicating the result of the asynchronous operation.
    ///
    pub async fn serialize(&self, mut buffer: impl AsyncWrite + Unpin) -> anyhow::Result<()> {
        let guard = self.get_data_guard();
        let count_all = guard.count.load(Ordering::Relaxed);
        buffer.write_i16(count_all).await?;
        let mut set: HashMap<u32, u32> = HashMap::with_capacity(1024);
        let mut entry: i32 = -1;
        let mut palette = Vec::with_capacity(4096);
        for i in guard.data.iter() {
            let id: u32 = match set.get(i) {
                None => unsafe {
                    entry += 1;
                    if entry >= 1024 {
                        palette = guard.data.to_vec();
                        break;
                    }
                    set.insert_unique_unchecked(*i, entry as u32);
                    entry as u32
                },
                Some(id) => *id,
            };
            palette.push(id);
        }
        if set.len() == 1 {
            buffer.write_u8(0).await?;
            buffer
                .write_var_int(unsafe { *palette.get_unchecked(0) as i32 })
                .await?;
            buffer.write_var_int(0).await?;
            return Ok(());
        }
        let bit_len = (palette.len() as f32).log2().ceil() as u8;
        buffer.write_u8(bit_len).await?;
        let entry_per_long = 64 / bit_len as i32;
        let data_array_len = 4096 / entry_per_long;
        buffer.write_var_int(data_array_len).await?;
        for i in 0..data_array_len {
            let mut long = 0;
            for j in 0..entry_per_long {
                let id = unsafe { *palette.get_unchecked((i * entry_per_long + j) as usize) };
                long |= (id as u64) << (j * bit_len as i32);
            }
            buffer.write_u64(long).await?;
        }
        Ok(())
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SectionDataGuard<'a> {
    pub data: MutexGuard<'a, [u32; 4096]>,
    pub count: &'a AtomicI16,
}

impl SectionDataGuard<'_> {
    /// Retrieves the state value at the specified coordinates
    ///
    /// # Arguments
    /// - `x`: The x-coordinate
    /// - `y`: The y-coordinate
    /// - `z`: The z-coordinate
    ///
    /// # Returns
    /// The state value at the given coordinates
    ///
    /// # Description
    /// This function computes the index based on x, y,
    /// and z coordinates and retrieves the state value from the internal data structure.
    pub fn get_state(&self, x: u32, y: u32, z: u32) -> u32 {
        let index = (x + 16 * (y + 16 * z)) as usize;
        self.data[index]
    }
    /// Gets the state at the given coordinates (x, y, z) unsafely.
    ///
    /// # Safety
    /// - The caller must ensure that the coordinates (x, y, z)
    ///   do not exceed the bounds of the internal data array.
    /// - This function does not check the validity of the coordinates,
    ///   and incorrect coordinates may lead to undefined behavior.
    ///
    /// # Parameters
    /// - `x`: The x coordinate, ranging from 0 to 15.
    /// - `y`: The y coordinate, ranging from 0 to 15.
    /// - `Z`: The z coordinate, ranging from 0 to 15.
    ///
    /// # Returns
    /// - Returns the state value at the given coordinates.
    pub unsafe fn get_state_unchecked(&self, x: u32, y: u32, z: u32) -> u32 {
        let index = (x + 16 * (y + 16 * z)) as usize;
        *self.data.get_unchecked(index)
    }
    /// Sets the state at the given coordinates.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate, representing the horizontal position in 3D space.
    /// - `Y`: The y-coordinate, representing the vertical position in 3D space.
    /// - `Z`: The z-coordinate, representing the depth position in 3D space.
    /// - `State`: The new state value to set at the given position.
    ///
    /// # Description
    /// This function calculates the index based on the 3D coordinates
    /// and sets the state of that index in the data array.
    pub fn set_state(&mut self, x: u32, y: u32, z: u32, state: u32) {
        let index = (x + 16 * (y + 16 * z)) as usize;
        if state != 0 && unsafe { self.data.get_unchecked(index) == &0 } {
            self.count.fetch_add(1, Ordering::Relaxed);
        } else if state == 0 && unsafe { self.data.get_unchecked(index) != &0 } {
            self.count.fetch_sub(1, Ordering::Relaxed);
        }
        self.data[index] = state;
    }
}
