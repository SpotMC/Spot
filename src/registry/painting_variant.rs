use dashmap::DashMap;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

lazy_static! {
    pub static ref PAINTING_VARIANTS: DashMap<String, PaintingVariant> = init_registry_map();
}

fn init_registry_map() -> DashMap<String, PaintingVariant> {
    let map: DashMap<String, PaintingVariant> = DashMap::new();
    PaintingVariant::insert(&map, "minecraft:kebab", 1, 1);
    PaintingVariant::insert(&map, "minecraft:aztec", 1, 1);
    PaintingVariant::insert(&map, "minecraft:aztec2", 1, 1);
    PaintingVariant::insert(&map, "minecraft:alban", 1, 1);
    PaintingVariant::insert(&map, "minecraft:bomb", 1, 1);
    PaintingVariant::insert(&map, "minecraft:plant", 1, 1);

    PaintingVariant::insert(&map, "minecraft:wasteland", 1, 1);
    PaintingVariant::insert(&map, "minecraft:meditative", 1, 1);
    PaintingVariant::insert(&map, "minecraft:wanderer", 1, 2);
    PaintingVariant::insert(&map, "minecraft:graham", 1, 2);

    PaintingVariant::insert(&map, "minecraft:prairie_ride", 1, 2);
    PaintingVariant::insert(&map, "minecraft:pool", 2, 1);
    PaintingVariant::insert(&map, "minecraft:courbet", 2, 1);
    PaintingVariant::insert(&map, "minecraft:sunset", 2, 1);
    PaintingVariant::insert(&map, "minecraft:sea", 2, 1);
    PaintingVariant::insert(&map, "minecraft:creebet", 2, 1);

    PaintingVariant::insert(&map, "minecraft:match", 2, 2);
    PaintingVariant::insert(&map, "minecraft:bust", 2, 2);
    PaintingVariant::insert(&map, "minecraft:stage", 2, 2);
    PaintingVariant::insert(&map, "minecraft:void", 2, 2);
    PaintingVariant::insert(&map, "minecraft:skull_and_roses", 2, 2);
    PaintingVariant::insert(&map, "minecraft:wither", 2, 2);
    PaintingVariant::insert(&map, "minecraft:baroque", 2, 2);
    PaintingVariant::insert(&map, "minecraft:humble", 2, 2);

    PaintingVariant::insert(&map, "minecraft:bouquet", 3, 3);
    PaintingVariant::insert(&map, "minecraft:cavebird", 3, 3);
    PaintingVariant::insert(&map, "minecraft:cotan", 3, 3);
    PaintingVariant::insert(&map, "minecraft:endboss", 3, 3);
    PaintingVariant::insert(&map, "minecraft:fern", 3, 3);
    PaintingVariant::insert(&map, "minecraft:owlemons", 3, 3);
    PaintingVariant::insert(&map, "minecraft:sunflower", 3, 3);
    PaintingVariant::insert(&map, "minecraft:tides", 3, 3);

    PaintingVariant::insert(&map, "minecraft:backyard", 3, 4);
    PaintingVariant::insert(&map, "minecraft:pond", 3, 4);

    PaintingVariant::insert(&map, "minecraft:fighters", 4, 2);
    PaintingVariant::insert(&map, "minecraft:changing", 4, 2);
    PaintingVariant::insert(&map, "minecraft:finding", 4, 2);
    PaintingVariant::insert(&map, "minecraft:lowmist", 4, 2);
    PaintingVariant::insert(&map, "minecraft:passage", 4, 2);

    PaintingVariant::insert(&map, "minecraft:skeleton", 4, 3);
    PaintingVariant::insert(&map, "minecraft:donkey_kong", 4, 3);

    PaintingVariant::insert(&map, "minecraft:pointer", 4, 4);
    PaintingVariant::insert(&map, "minecraft:pigscene", 4, 4);
    PaintingVariant::insert(&map, "minecraft:burning_skull", 4, 4);
    PaintingVariant::insert(&map, "minecraft:unpacked", 4, 4);
    PaintingVariant::insert(&map, "minecraft:orb", 4, 4);
    map
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaintingVariant {
    pub width: i32,
    pub height: i32,
    pub asset_id: String,
}
impl PaintingVariant {
    fn insert(map: &DashMap<String, PaintingVariant>, asset_id: &str, width: i32, height: i32) {
        let asset_id: String = asset_id.parse().unwrap();
        map.insert(
            asset_id.to_owned(),
            PaintingVariant {
                width,
                height,
                asset_id,
            },
        );
    }
}
