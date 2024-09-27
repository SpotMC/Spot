use crate::util::to_chunk_yzx;

pub struct Chunk {
    pub(crate) data: Vec<u32>,
    pub(crate) height: i32,
}

impl Chunk {
    pub fn new(height: i32) -> Chunk {
        let size = 16 * 16 * height as usize;
        let data: Vec<u32> = vec![0; size];
        Chunk { data, height }
    }
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        if y < 0 || y >= self.height {
            return None;
        }
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return None;
        }
        let yzx = to_chunk_yzx(x, y, z);
        Some(*self.data.get(yzx)?)
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u32) {
        if y < 0 || y >= self.height {
            return;
        }
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return;
        }
        let yzx = to_chunk_yzx(x, y, z);
        if yzx >= self.data.capacity() {
            return;
        }
        self.data.insert(yzx, block);
    }
}
