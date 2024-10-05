use crate::world::dimension::Dimension;
use crate::WORLD;
use parking_lot::{Mutex, MutexGuard};

pub struct Chunk {
    pub(crate) data: Vec<Section>,
    pub(crate) pos: u64,
    pub(crate) height: i32,
    pub(crate) idx: u32,
}

impl Chunk {
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
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let idx = check_pos(x, y, z, self.height)?;
        let section = self.data.get(idx)?;
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(unsafe { section.get_state_unchecked(x as u32, sy, z as u32) })
    }

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

    pub fn get_section(&self, y: usize) -> Option<SectionDataGuard> {
        Some(self.data.get(y / 16)?.get_data_guard())
    }

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
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let idx = check_pos(x, y, z, self.height)?;
        let section = self.data.get(idx)?;
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(unsafe { section.get_state_unchecked(x as u32, sy, z as u32) })
    }

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
}

impl Section {
    pub fn new() -> Section {
        Section {
            data: Mutex::new([0; 4096]),
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
        self.data.lock()[index] = state;
    }

    #[inline]
    pub fn get_data_guard(&self) -> SectionDataGuard {
        SectionDataGuard {
            data: self.data.lock(),
        }
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SectionDataGuard<'a> {
    pub data: MutexGuard<'a, [u32; 4096]>,
}

impl SectionDataGuard<'_> {
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

    pub fn set_state(&mut self, x: u32, y: u32, z: u32, state: u32) {
        let index = (x + 16 * (y + 16 * z)) as usize;
        self.data[index] = state;
    }
}
