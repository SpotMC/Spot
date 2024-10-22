use crate::world::chunk::Chunk;
use crate::world::gen::Worldgen;

pub struct SuperFlatWorldgen {
    start_y: u32,
    blocks: Vec<u32>,
}
impl Worldgen for SuperFlatWorldgen {
    fn gen(&self, chunk: Chunk) -> Chunk {
        let mut guard = chunk.get_guard();
        for y in 0..self.blocks.len() {
            let block = unsafe { self.blocks.get_unchecked(y) };
            let y = y as i32 + self.start_y as i32;
            for x in 0..16 {
                for z in 0..16 {
                    guard.set_block(x, y, z, *block).unwrap();
                }
            }
        }
        drop(guard);
        chunk
    }
}

impl SuperFlatWorldgen {
    pub fn new(start_y: u32, blocks: Vec<u32>) -> SuperFlatWorldgen {
        SuperFlatWorldgen { start_y, blocks }
    }
}

pub struct VoidWorldgen;
impl Worldgen for VoidWorldgen {
    fn gen(&self, chunk: Chunk) -> Chunk {
        chunk
    }
}
