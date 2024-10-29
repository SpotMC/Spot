use crate::entity::player::Player;
use crate::entity::Entity;
use crate::util::arc_channel::ArcChannel;
use crate::util::io::WriteExt;
use crate::util::raw::Raw;
use crate::world::chunk::ChunkUpdate::BlockChange;
use crate::world::dimension::Dimension;
use crate::world::height_map::HeightMap;
use anyhow::anyhow;
use arc_swap::ArcSwapOption;
use bit_set::BitSet;
use bytes::BytesMut;
use dashmap::DashMap;
use hashbrown::HashMap;
use parking_lot::{Mutex, MutexGuard, RwLock};
use rayon::prelude::*;
use simdnbt::owned::NbtTag::LongArray;
use simdnbt::owned::{BaseNbt, NbtCompound};
use std::sync::atomic::{AtomicI16, Ordering};
use std::sync::{Arc, Weak};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct Chunk {
    world_surface: Mutex<HeightMap>,
    motion_blocking: Mutex<HeightMap>,
    data: Vec<Section>,
    pos: u64,
    height: i32,
    dimension: Raw<DashMap<u64, Weak<Chunk>>>,
    channel: RwLock<ArcChannel<ChunkUpdate>>,
    cache: ArcSwapOption<BytesMut>,
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
            world_surface: Mutex::new(HeightMap::new(height)),
            motion_blocking: Mutex::new(HeightMap::new(height)),
            data,
            height,
            dimension: Raw::from(&dimension.chunks),
            pos,
            channel: RwLock::default(),
            cache: ArcSwapOption::empty(),
        }
    }
    /// Retrieves the mutex guard for the world-surface height map.
    ///
    /// # Returns
    /// * `MutexGuard<HeightMap>` - A guard that provides mutable access to the height map,
    ///   ensuring that no other thread can modify it while the guard is held.
    #[inline]
    pub fn get_world_surface(&self) -> MutexGuard<HeightMap> {
        self.world_surface.lock()
    }
    /// Retrieves the mutex guard for the motion-blocking height map.
    ///
    /// # Returns
    /// * `MutexGuard<HeightMap>` - A guard that provides mutable access to the height map,
    ///   ensuring that no other thread can modify it while the guard is held.
    #[inline]
    pub fn get_motion_blocking(&self) -> MutexGuard<HeightMap> {
        self.motion_blocking.lock()
    }

    /// Gets chunk's position coordinates.
    #[inline]
    pub fn get_position(&self) -> (i32, i32) {
        ((self.pos >> 32) as i32, (self.pos & 0xFFFFFFFF) as i32)
    }

    /// Retrieves the serialized chunk data.
    ///
    /// This function attempts to retrieve the serialized data from a cache.
    /// If the cache is empty,
    /// it performs the serialization and stores the result in the cache for future use.
    ///
    /// # Returns
    ///
    /// - `Ok(Arc<BytesMut>)`: The serialized data wrapped in an `Arc<BytesMut>`.
    /// - `Err(anyhow::Error)`: An error if serialization or cache operations fail.
    pub async fn get_serialized(&self) -> anyhow::Result<Arc<BytesMut>> {
        match self.cache.load().as_ref() {
            Some(cache) => Ok(cache.clone()),
            None => {
                let mut buf = Vec::with_capacity(2048);
                self.serialize(&mut buf).await?;
                let buf = Arc::from(BytesMut::from(buf.as_slice()));
                self.cache.store(Some(buf.clone()));
                Ok(buf)
            }
        }
    }

    pub fn player_enter(&self, player: &mut Player) {
        player.recv.add(
            self.pos,
            self.channel
                .write()
                .subscribe_with_id(player.get_eid() as usize),
        );
    }
    pub fn player_exit(&self, player: &mut Player) {
        if let Some(recv) = player.recv.remove(self.pos) {
            self.channel.write().remove(&recv);
        }
    }
    #[inline]
    fn invalidate_cache(&self) {
        self.cache.store(None);
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
    pub fn set_block(&self, x: i32, y: i32, z: i32, block: u32) -> anyhow::Result<()> {
        if !(0..16).contains(&x) {
            return Err(anyhow!("Invalid x coord: {}", x));
        } else if !(0..16).contains(&z) {
            return Err(anyhow!("Invalid z coord: {}", z));
        } else if y < 0 || y >= self.height {
            return Err(anyhow!("Invalid y coord: {}", y));
        }
        self.invalidate_cache();
        self.channel.read().broadcast(BlockChange(x, y, z, block));
        let idx = y as usize / 16;
        let section = self.data.get(idx).ok_or(anyhow!("Invalid position"))?;
        let sy = ((y as usize) - (16 * idx)) as u32;
        section.set_state(x as u32, sy, z as u32, block);
        Ok(())
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
        self.invalidate_cache();
        Some(self.data.get(y / 16)?.get_data_guard())
    }
    /// Get a guard object that provides pre-locked access to the chunk data.
    pub fn get_guard(&self) -> ChunkGuard<'_> {
        let sections = self.data.len();
        let mut data = Vec::with_capacity(sections);
        self.invalidate_cache();
        for i in 0..sections {
            data.push(self.data.get(i).unwrap().get_data_guard());
        }
        ChunkGuard {
            data,
            height: self.height,
        }
    }

    /// Retrieves the sky light value at the specified position.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate of the block.
    /// * `y` - The y-coordinate of the block.
    /// * `z` - The z-coordinate of the block.
    ///
    /// # Returns
    /// * `Option<u8>` - The light value of the block at the specified position, or `None`
    ///   if the position is invalid.
    pub fn get_sky_light(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        let idx = check_pos(x, y, z, self.height)?;
        let section = unsafe { self.data.get_unchecked(idx) };
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(section.get_sky_light(x as u32, sy, z as u32))
    }

    /// Sets the sky light value at a specific position.
    ///
    /// This function will invalidate the cache.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the block, valid range is 0 to 15
    /// * `y` - The y coordinate of the block, valid range is 0 to `self.height - 1`
    /// * `z` - The z coordinate of the block, valid range is 0 to 15
    /// * `light` - The light value to set, valid range is 0 to 15
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful
    /// * `Err(anyhow::Error)` - If any of the coordinates are out of bounds
    ///
    /// # Errors
    ///
    /// * Returns an error if `x` is not in the range 0 to 15
    /// * Returns an error if `z` is not in the range 0 to 15
    /// * Returns an error if `y` is not in the range 0 to `self.height - 1`
    pub fn set_sky_light(&self, x: i32, y: i32, z: i32, light: u8) -> anyhow::Result<()> {
        if !(0..16).contains(&x) {
            return Err(anyhow!("Invalid x coord: {}", x));
        } else if !(0..16).contains(&z) {
            return Err(anyhow!("Invalid z coord: {}", z));
        } else if y < 0 || y >= self.height {
            return Err(anyhow!("Invalid y coord: {}", y));
        }
        self.invalidate_cache();
        let idx = y as usize / 16;
        let section = unsafe { self.data.get_unchecked(idx) };
        let sy = ((y as usize) - (16 * idx)) as u32;
        section.set_sky_light(x as u32, sy, z as u32, light);
        Ok(())
    }

    /// Retrieves the block light value at the specified position.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate of the block.
    /// * `y` - The y-coordinate of the block.
    /// * `z` - The z-coordinate of the block.
    ///
    /// # Returns
    /// * `Option<u8>` - The light value of the block at the specified position, or `None`
    ///   if the position is invalid.
    pub fn get_block_light(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        let idx = check_pos(x, y, z, self.height)?;
        let section = unsafe { self.data.get_unchecked(idx) };
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(section.get_block_light(x as u32, sy, z as u32))
    }

    /// Sets the block light value at a specific position.
    ///
    /// This function will invalidate the cache.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the block, valid range is 0 to 15
    /// * `y` - The y coordinate of the block, valid range is 0 to `self.height - 1`
    /// * `z` - The z coordinate of the block, valid range is 0 to 15
    /// * `light` - The light value to set, valid range is 0 to 15
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful
    /// * `Err(anyhow::Error)` - If any of the coordinates are out of bounds
    ///
    /// # Errors
    ///
    /// * Returns an error if `x` is not in the range 0 to 15
    /// * Returns an error if `z` is not in the range 0 to 15
    /// * Returns an error if `y` is not in the range 0 to `self.height - 1`
    pub fn set_block_light(&self, x: i32, y: i32, z: i32, light: u8) -> anyhow::Result<()> {
        if !(0..16).contains(&x) {
            return Err(anyhow!("Invalid x coord: {}", x));
        } else if !(0..16).contains(&z) {
            return Err(anyhow!("Invalid z coord: {}", z));
        } else if y < 0 || y >= self.height {
            return Err(anyhow!("Invalid y coord: {}", y));
        }
        self.invalidate_cache();
        let idx = y as usize / 16;
        let section = unsafe { self.data.get_unchecked(idx) };
        let sy = ((y as usize) - (16 * idx)) as u32;
        section.set_block_light(x as u32, sy, z as u32, light);
        Ok(())
    }

    async fn serialize<W: AsyncWrite + Unpin>(&self, buffer: &mut W) -> anyhow::Result<()> {
        // Chunk Position
        buffer.write_u64(self.pos).await?;
        // Height Map
        let mut height_map = Vec::with_capacity(256);
        BaseNbt::new(
            "",
            NbtCompound::from_values(vec![
                (
                    "MOTION_BLOCKING".into(),
                    LongArray(self.motion_blocking.lock().serialize()?),
                ),
                (
                    "WORLD_SURFACE".into(),
                    LongArray(self.world_surface.lock().serialize()?),
                ),
            ]),
        )
        .write_unnamed(&mut height_map);
        buffer.write_all(&height_map).await?;
        // Block Data
        let mut section_data = Vec::with_capacity(2048);
        for section in self.data.iter() {
            section
                .get_data_guard()
                .serialize(&mut section_data)
                .await?;
        }
        buffer.write_var_int(section_data.len() as i32).await?;
        buffer.write_all(&section_data).await?;
        // Block Entities (WIP)
        buffer.write_var_int(0).await?;
        // Light (WIP)
        let sky_light_mask = BitSet::with_capacity(self.data.len() + 2);
        let block_light_mask = BitSet::with_capacity(self.data.len() + 2);
        let mut empty_sky_light_mask = BitSet::with_capacity(self.data.len() + 2);
        let mut empty_block_light_mask = BitSet::with_capacity(self.data.len() + 2);

        empty_block_light_mask.insert(0);
        empty_sky_light_mask.insert(0);
        empty_block_light_mask.insert(self.data.len() + 1);
        empty_sky_light_mask.insert(self.data.len() + 1);

        let sky_light_mask = Mutex::from(sky_light_mask);
        let block_light_mask = Mutex::from(block_light_mask);
        let empty_sky_light_mask = Mutex::from(empty_sky_light_mask);
        let empty_block_light_mask = Mutex::from(empty_block_light_mask);

        let sky = Mutex::from(vec![false; self.data.len()]);
        let block = Mutex::from(vec![false; self.data.len()]);
        // TODO :BitSet should be replaced by valence's BitSet
        self.data
            .par_iter()
            .enumerate()
            .for_each(|(index, section)| {
                let guard = section.get_light_guard();
                let mut sky_not_empty = false;
                let mut block_not_empty = false;

                for value in guard.sky_light.iter() {
                    if *value != 0 {
                        sky_not_empty = true;
                        break;
                    }
                }
                for value in guard.block_light.iter() {
                    if *value != 0 {
                        block_not_empty = true;
                        break;
                    }
                }

                if sky_not_empty {
                    sky.lock()[index] = true;
                    sky_light_mask.lock().insert(index);
                    empty_sky_light_mask.lock().remove(index);
                } else {
                    sky_light_mask.lock().remove(index);
                    empty_sky_light_mask.lock().insert(index);
                }
                if block_not_empty {
                    block.lock()[index] = true;
                    block_light_mask.lock().insert(index);
                    empty_block_light_mask.lock().remove(index);
                } else {
                    block_light_mask.lock().remove(index);
                    empty_block_light_mask.lock().insert(index);
                }
            });
        let sky_mask = sky_light_mask.into_inner();
        let block_mask = block_light_mask.into_inner();
        let block = block.into_inner();
        let sky = sky.into_inner();

        buffer.write_bitset(&sky_mask).await?;
        buffer.write_bitset(&block_mask).await?;
        buffer
            .write_bitset(&empty_sky_light_mask.into_inner())
            .await?;
        buffer
            .write_bitset(&empty_block_light_mask.into_inner())
            .await?;
        buffer.write_var_int(sky_mask.len() as i32).await?;
        for (index, section) in self.data.iter().enumerate() {
            if sky[index] {
                let temp = {
                    let temp = section.sky_light.lock();
                    temp.to_vec()
                };
                buffer.write_var_int(2048).await?;
                buffer.write_all(&temp).await?;
            }
        }
        buffer.write_var_int(block_mask.len() as i32).await?;
        for (index, section) in self.data.iter().enumerate() {
            if block[index] {
                let temp = {
                    let temp = section.block_light.lock();
                    temp.to_vec()
                };
                buffer.write_var_int(2048).await?;
                buffer.write_all(&temp).await?;
            }
        }
        Ok(())
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if let Some(chunks) = self.dimension.get() {
            chunks.remove(&self.pos);
        }
    }
}

pub enum ChunkUpdate {
    BlockChange(i32, i32, i32, u32),
}

pub struct ChunkGuard<'a> {
    data: Vec<SectionDataGuard<'a>>,
    pub height: i32,
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
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) -> anyhow::Result<()> {
        if !(0..16).contains(&x) {
            return Err(anyhow!("Invalid x coord: {}", x));
        } else if !(0..16).contains(&z) {
            return Err(anyhow!("Invalid z coord: {}", z));
        } else if y < 0 || y >= self.height {
            return Err(anyhow!("Invalid y coord: {}", y));
        }
        let idx = y as usize / 16;
        let section = unsafe { self.data.get_unchecked_mut(idx) };
        let sy = ((y as usize) - (16 * idx)) as u32;
        section.set_state(x as u32, sy, z as u32, block);
        Ok(())
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
    sky_light: Mutex<[u8; 2048]>,
    block_light: Mutex<[u8; 2048]>,
    data: Mutex<[u32; 4096]>,
    block_count: AtomicI16,
}

impl Section {
    pub fn new() -> Section {
        Section {
            sky_light: Mutex::new([0; 2048]),
            block_light: Mutex::new([0; 2048]),
            data: Mutex::new([0; 4096]),
            block_count: AtomicI16::new(0),
        }
    }

    pub fn set_sky_light(&self, x: u32, y: u32, z: u32, light: u8) {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let mut lock = self.sky_light.lock();
        let base = lock[index];
        if x % 2 == 0 {
            lock[index] = (light << 4) | (base & 0xF);
        } else {
            lock[index] = (light & 0xF) | (base & 0xF0);
        }
    }

    pub fn get_sky_light(&self, x: u32, y: u32, z: u32) -> u8 {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let lock = self.sky_light.lock();
        let value = lock[index];
        if x % 2 == 0 {
            value >> 4
        } else {
            value
        }
    }

    pub fn set_block_light(&self, x: u32, y: u32, z: u32, light: u8) {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let mut lock = self.block_light.lock();
        let base = lock[index];
        if x % 2 == 0 {
            lock[index] = (light << 4) | (base & 0xF);
        } else {
            lock[index] = (light & 0xF) | (base & 0xF0)
        }
    }

    pub fn get_block_light(&self, x: u32, y: u32, z: u32) -> u8 {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let lock = self.block_light.lock();
        let value = lock[index];
        if x % 2 == 0 {
            value >> 4
        } else {
            value
        }
    }

    pub fn get_state(&self, x: u32, y: u32, z: u32) -> u32 {
        let index = ((y << 8) | (z << 4) | x) as usize;
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
        let index = ((y << 8) | (z << 4) | x) as usize;
        *self.data.lock().get_unchecked(index)
    }

    #[inline]
    pub fn set_state(&self, x: u32, y: u32, z: u32, state: u32) {
        let index = ((y << 8) | (z << 4) | x) as usize;
        let mut data = self.data.lock();
        if state != 0 && unsafe { data.get_unchecked(index) == &0 } {
            self.block_count.fetch_add(1, Ordering::SeqCst);
        } else if state == 0 && unsafe { data.get_unchecked(index) != &0 } {
            self.block_count.fetch_sub(1, Ordering::SeqCst);
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

    #[inline]
    pub fn get_light_guard(&self) -> SectionLightGuard {
        SectionLightGuard {
            sky_light: self.sky_light.lock(),
            block_light: self.block_light.lock(),
        }
    }

    pub fn get_block_count(&self) -> i16 {
        self.block_count.load(Ordering::Acquire)
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SectionLightGuard<'a> {
    pub sky_light: MutexGuard<'a, [u8; 2048]>,
    pub block_light: MutexGuard<'a, [u8; 2048]>,
}

impl SectionLightGuard<'_> {
    pub fn get_sky_light(&self, x: u32, y: u32, z: u32) -> u8 {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let base = self.sky_light[index];
        if x % 2 == 0 {
            base >> 4
        } else {
            base
        }
    }

    pub fn get_block_light(&self, x: u32, y: u32, z: u32) -> u8 {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let base = self.block_light[index];
        if x % 2 == 0 {
            base >> 4
        } else {
            base
        }
    }

    pub fn set_sky_light(&mut self, x: u32, y: u32, z: u32, light: u8) {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let base = self.sky_light[index];
        if x % 2 == 0 {
            self.sky_light[index] = (light << 4) | (base & 0xF);
        } else {
            self.sky_light[index] = (light & 0xF) | (base & 0xF0)
        }
    }

    pub fn set_block_light(&mut self, x: u32, y: u32, z: u32, light: u8) {
        let index = (((y << 8) | (z << 4) | x) / 2) as usize;
        let base = self.block_light[index];
        if x % 2 == 0 {
            self.block_light[index] = (light << 4) | (base & 0xF);
        } else {
            self.block_light[index] = (light & 0xF) | (base & 0xF0)
        }
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
        let index = ((y << 8) | (z << 4) | x) as usize;
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
        let index = ((y << 8) | (z << 4) | x) as usize;
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
        let index = ((y << 8) | (z << 4) | x) as usize;
        if state != 0 && unsafe { self.data.get_unchecked(index) == &0 } {
            self.count.fetch_add(1, Ordering::SeqCst);
        } else if state == 0 && unsafe { self.data.get_unchecked(index) != &0 } {
            self.count.fetch_sub(1, Ordering::SeqCst);
        }
        self.data[index] = state;
    }
    pub fn get_block_count(&self) -> i16 {
        self.count.load(Ordering::SeqCst)
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
    pub async fn serialize<W: AsyncWrite + Unpin>(&self, buffer: &mut W) -> anyhow::Result<()> {
        let count_all = self.count.load(Ordering::SeqCst);
        buffer.write_i16(count_all).await?;
        let mut set: HashMap<u32, u32> = HashMap::with_capacity(1024);
        let mut entry: i32 = -1;
        let mut palette = Vec::with_capacity(4096);
        for i in self.data.iter() {
            let id: u32 = match set.get(i) {
                None => unsafe {
                    entry += 1;
                    if entry >= 1024 {
                        palette = self.data.to_vec();
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
        // TODO: Biome palette
        Ok(())
    }
}
