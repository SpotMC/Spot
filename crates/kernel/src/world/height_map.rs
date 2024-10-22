use crate::world::chunk::ChunkGuard;

#[inline]
fn generate_height_map(chunk: &mut ChunkGuard, map: &mut HeightMap, op: impl Fn(u32) -> bool) {
    for x in 0..16 {
        for z in 0..16 {
            let mut y = chunk.height;
            while y > 0 {
                y -= 1;
                let block = chunk.get_block(x, y, z).unwrap();
                if op(block) {
                    map.set_value(x, z, y as u16);
                    break;
                }
            }
        }
    }
}

pub struct HeightMap {
    pub(crate) height_map: [u16; 256],
}

impl Default for HeightMap {
    fn default() -> Self {
        Self::new()
    }
}

impl HeightMap {
    pub fn new() -> HeightMap {
        HeightMap {
            height_map: [0; 256],
        }
    }
    pub fn get_value(&self, x: i32, z: i32) -> Option<u16> {
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return None;
        }
        Some(unsafe { *self.height_map.get_unchecked((x << 4 | z) as usize) })
    }

    pub fn set_value(&mut self, x: i32, z: i32, value: u16) {
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return;
        }
        unsafe { *self.height_map.get_unchecked_mut((x << 4 | z) as usize) = value };
    }
}
