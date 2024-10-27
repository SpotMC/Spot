use crate::world::chunk::ChunkGuard;
use anyhow::anyhow;

#[allow(dead_code)]
pub fn generate_height_map(
    chunk: &mut ChunkGuard,
    map: &mut HeightMap,
    op: impl Fn(u32) -> bool,
) -> anyhow::Result<()> {
    for x in 0..16 {
        for z in 0..16 {
            let mut y = chunk.height;
            while y > 0 {
                y -= 1;
                let block = chunk.get_block(x, y, z).unwrap();
                if op(block) {
                    map.set(x, z, y as u16)?;
                    break;
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
pub struct HeightMap {
    pub(crate) height_map: [u16; 256],
    bit_per_entry: u32,
    mask: u32,
    u: u32,
}

impl HeightMap {
    pub fn new(height: i32) -> HeightMap {
        let bit_per_entry = height.ilog2() + 1;
        HeightMap {
            height_map: [0; 256],
            bit_per_entry,
            mask: (1 << height.ilog2()) - 1,
            u: 64 / bit_per_entry,
        }
    }
    pub fn get(&self, x: i32, z: i32) -> Option<u16> {
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
            return None;
        }
        Some(unsafe { *self.height_map.get_unchecked((x << 4 | z) as usize) })
    }

    pub fn set(&mut self, x: i32, z: i32, value: u16) -> anyhow::Result<()> {
        if !(0..16).contains(&x) {
            return Err(anyhow!("Invalid x coord: {}", x));
        } else if !(0..16).contains(&z) {
            return Err(anyhow!("Invalid z coord: {}", z));
        }
        unsafe { *self.height_map.get_unchecked_mut((x << 4 | z) as usize) = value };
        Ok(())
    }

    pub fn serialize(&self) -> anyhow::Result<Vec<i64>> {
        let mut result = Vec::with_capacity(self.height_map.len() / 8 + 1);
        for i in 0..self.height_map.len() / 8 {
            let mut value = 0;
            for j in 0..8 {
                value |= (self.height_map[i * 8 + j] as u64) << (j * self.bit_per_entry as usize);
            }
            result.push(value as i64);
        }
        Ok(result)
    }
}

impl AsRef<[u16; 256]> for HeightMap {
    fn as_ref(&self) -> &[u16; 256] {
        &self.height_map
    }
}

impl AsMut<[u16; 256]> for HeightMap {
    fn as_mut(&mut self) -> &mut [u16; 256] {
        &mut self.height_map
    }
}
