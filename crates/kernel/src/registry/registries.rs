use crate::block::{
    air::*, bedrock::*, birch_leaves::*, birch_log::*, cave_air::*, deepslate::*, dirt::*,
    grass_block::*, oak_leaves::*, oak_log::*, register_block, stone::*, Block,
};
use crate::item::block_items::*;
use crate::item::{register_block_item, BlockItem};

pub(crate) fn register_vanilla() {
    let grass_block = GrassBlock::new();
    register(
        GrassBlockItem::new(grass_block.get_block_id()),
        grass_block,
        GRASS_BLOCK,
    );

    let stone = Stone::new();
    register(StoneItem::new(stone.get_block_id()), stone, STONE);

    let dirt = Dirt::new();
    register(DirtItem::new(dirt.get_block_id()), dirt, DIRT);

    let bedrock = Bedrock::new();
    register(BedrockItem::new(bedrock.get_block_id()), bedrock, BEDROCK);

    let deepslate = DeepSlate::new();
    register(
        DeepslateItem::new(deepslate.get_block_id()),
        deepslate,
        DEEPSLATE,
    );

    let oak_log = OakLog::new();
    register(OakLogItem::new(oak_log.get_block_id()), oak_log, OAK_LOG);

    let oak_leaves = OakLeaves::new();
    register(
        OakLeavesItem::new(oak_leaves.get_block_id()),
        oak_leaves,
        OAK_LEAVES,
    );

    let birch_log = BirchLog::new();
    register(
        BirchLogItem::new(birch_log.get_block_id()),
        birch_log,
        BIRCH_LOG,
    );

    let birch_leaves = BirchLeaves::new();
    register(
        BirchLeavesItem::new(birch_leaves.get_block_id()),
        birch_leaves,
        BIRCH_LEAVES,
    );

    register_block(CAVE_AIR, Box::new(CaveAir::new()));
    register_block(AIR, Box::new(Air::new()));
}

fn register<B: Block, I: BlockItem>(item: I, block: B, id: &str) {
    register_block_item(id, block.get_block_id(), Box::new(item));
    register_block(id, Box::new(block));
}
