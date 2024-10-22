mod chunk {
    #[test]
    fn chunk() {
        use crate::world::chunk::Chunk;
        use crate::world::dimension::Dimension;
        let chunk = Chunk::new(
            &Dimension::new(
                crate::registry::dimension_type::DIMENSION_TYPES
                    .get("minecraft:overworld")
                    .unwrap()
                    .clone(),
                "overworld".to_string(),
                0,
            ),
            0,
        );
        chunk.set_block(0, 0, 0, 9).unwrap();
        chunk.set_block(11, 45, 14, 9).unwrap();
        chunk.set_block(15, 383, 15, 9).unwrap();
        assert_eq!(chunk.get_block(0, 0, 0), Some(9));
        assert_eq!(chunk.get_block(11, 45, 14), Some(9));
        assert_eq!(chunk.get_block(15, 383, 15), Some(9));
        assert_eq!(chunk.get_block(0, 1, 0), Some(0));
    }
}
mod gen {
    #[test]
    fn worldgen() {
        use crate::block::BLOCKS_BY_NAME;
        use crate::registry::registries::register_vanilla;
        use crate::world::chunk::Chunk;
        use crate::world::dimension::Dimension;
        use crate::world::gen::impls::SuperFlatWorldgen;
        use crate::world::gen::Worldgen;

        register_vanilla();
        let bedrock = *BLOCKS_BY_NAME.get("minecraft:bedrock").unwrap().value();
        let dirt = *BLOCKS_BY_NAME.get("minecraft:dirt").unwrap().value();
        let grass_block = *BLOCKS_BY_NAME.get("minecraft:grass_block").unwrap().value();

        let chunk = SuperFlatWorldgen::new(0, vec![bedrock, dirt, grass_block]).gen(Chunk::new(
            &Dimension::new(
                crate::registry::dimension_type::DIMENSION_TYPES
                    .get("minecraft:overworld")
                    .unwrap()
                    .clone(),
                "overworld".to_string(),
                0,
            ),
            0,
        ));
        assert_eq!(chunk.get_block(0, 0, 0), Some(bedrock));
        assert_eq!(chunk.get_block(0, 1, 0), Some(dirt));
        assert_eq!(chunk.get_block(0, 2, 0), Some(grass_block));
        assert_eq!(chunk.get_block(0, 3, 0), Some(0));
        assert_eq!(chunk.get_block(15, 0, 15), Some(bedrock));
    }
}
mod dimension {
    #[test]
    fn dimension() {
        use crate::world::dimension::Dimension;
        let mut chunks = Vec::with_capacity(3);
        let dimension = Dimension::new(
            crate::registry::dimension_type::DIMENSION_TYPES
                .get("minecraft:overworld")
                .unwrap()
                .clone(),
            "overworld".to_string(),
            0,
        );
        chunks.push(dimension.set_block(1144657482, 319, -138848321, 9));
        chunks.push(dimension.set_block(1145, 14, 1919, 9));
        chunks.push(dimension.set_block(0, -64, 0, 9));
        assert_eq!(dimension.get_block(1144657482, 319, -138848321), Some(9));
        assert_eq!(dimension.get_block(1145, 14, 1919), Some(9));
        assert_eq!(dimension.get_block(0, -64, 0), Some(9));
    }
}
