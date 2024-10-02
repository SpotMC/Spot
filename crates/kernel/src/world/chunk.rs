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
        if y < 0 || y >= self.height {
            return None;
        }
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return None;
        }
        let idx = y as usize / 16;
        let section = self.data.get(idx);
        let sy = ((y as usize) - (16 * idx)) as u32;
        Some(section?.get_state(x as u32, sy, z as u32))
    }

    pub fn set_block(&self, x: i32, y: i32, z: i32, block: u32) {
        if y < 0 || y >= self.height {
            return;
        }
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return;
        }
        let idx = y as usize / 16;
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
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            WORLD
                .read()
                .dimensions
                .get(self.idx as usize)
                .unwrap()
                .chunks
                .remove(&self.pos);
        }
    }
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
    #[inline]
    pub fn get_state(&self, x: u32, y: u32, z: u32) -> u32 {
        let index = (x + 16 * (y + 16 * z)) as usize;
        self.data.lock()[index]
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
    pub fn get_state(&self, x: i32, y: i32, z: i32) -> u32 {
        let index = (x + 16 * (y + 16 * z)) as usize;
        self.data[index]
    }

    pub fn set_state(&mut self, x: i32, y: i32, z: i32, state: u32) {
        let index = (x + 16 * (y + 16 * z)) as usize;
        self.data[index] = state;
    }
}
