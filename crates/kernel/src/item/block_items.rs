use crate::block::bedrock::BEDROCK;
use crate::block::birch_leaves::BIRCH_LEAVES;
use crate::block::birch_log::BIRCH_LOG;
use crate::block::deepslate::DEEPSLATE;
use crate::block::dirt::DIRT;
use crate::block::grass_block::GRASS_BLOCK;
use crate::block::oak_leaves::OAK_LEAVES;
use crate::block::oak_log::OAK_LOG;
use crate::block::stone::STONE;
use crate::block_item_def;
use crate::item::*;

block_item_def!(StoneItem, STONE, ItemSettings::new());
block_item_def!(GrassBlockItem, GRASS_BLOCK, ItemSettings::new());
block_item_def!(DirtItem, DIRT, ItemSettings::new());
block_item_def!(BedrockItem, BEDROCK, ItemSettings::new());
block_item_def!(DeepslateItem, DEEPSLATE, ItemSettings::new());
block_item_def!(OakLogItem, OAK_LOG, ItemSettings::new());
block_item_def!(BirchLogItem, BIRCH_LOG, ItemSettings::new());
block_item_def!(OakLeavesItem, OAK_LEAVES, ItemSettings::new());
block_item_def!(BirchLeavesItem, BIRCH_LEAVES, ItemSettings::new());