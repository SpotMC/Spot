pub mod impls;

use crate::block::BLOCKS_BY_NAME;
use crate::world::chunk::Chunk;
use crate::world::gen::impls::SuperFlatWorldgen;
use hashbrown::HashMap;
use std::sync::LazyLock;

pub static IMPLEMENTS: LazyLock<HashMap<String, Box<dyn Worldgen>>> = LazyLock::new(|| {
    let mut map: HashMap<String, Box<dyn Worldgen>> = HashMap::with_capacity(5);
    map.insert(
        String::from("super_flat"),
        Box::new(SuperFlatWorldgen::new(
            0,
            vec![
                *BLOCKS_BY_NAME.get("minecraft:bedrock").unwrap().value(),
                *BLOCKS_BY_NAME.get("minecraft:dirt").unwrap().value(),
                *BLOCKS_BY_NAME.get("minecraft:grass_block").unwrap().value(),
            ],
        )),
    );
    map
});

pub trait Worldgen: Send + Sync {
    fn gen(&self, chunk: Chunk) -> Chunk;
}
