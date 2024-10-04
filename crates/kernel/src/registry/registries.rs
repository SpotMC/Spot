use crate::block::{
    air::*, birch_leaves::*, birch_log::*, cave_air::*, deepslate::*, dirt::*, grass_block::*,
    oak_leaves::*, oak_log::*, register_block, stone::*, Block,
};
use crate::item::dirt_item::DirtItem;
use crate::item::grass_block_item::GrassBlockItem;
use crate::item::stone_item::StoneItem;
use crate::item::{register_block_item, BlockItem};

pub(crate) fn register_vanilla() {
    {
        let grass_block = GrassBlock::new();
        register(
            GrassBlockItem::new(grass_block.get_block_id()),
            grass_block,
            GRASS_BLOCK,
        );
    }
    {
        let stone = Stone::new();
        register(StoneItem::new(stone.get_block_id()), stone, STONE);
    }
    {
        let dirt = Dirt::new();
        register(DirtItem::new(dirt.get_block_id()), dirt, DIRT);
    }
    register_block(DEEPSLATE, Box::new(DeepSlate::new()));
    register_block(OAK_LOG, Box::new(OakLog::new()));
    register_block(OAK_LEAVES, Box::new(OakLeaves::new()));
    register_block(BIRCH_LOG, Box::new(BirchLog::new()));
    register_block(BIRCH_LEAVES, Box::new(BirchLeaves::new()));
    register_block(CAVE_AIR, Box::new(CaveAir::new()));
    register_block(AIR, Box::new(Air::new()));
}

fn register<B: Block, I: BlockItem>(item: I, block: B, id: &str) {
    register_block_item(id, block.get_block_id(), Box::new(item));
    register_block(id, Box::new(block));
}
