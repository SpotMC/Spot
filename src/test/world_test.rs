mod chunk {
    #[test]
    fn chunk() {
        use crate::world::chunk::Chunk;
        let mut chunk = Chunk::new(384);
        chunk.set_block(0, 0, 0, 9);
        chunk.set_block(11, 45, 14, 9);
        chunk.set_block(15, 383, 15, 9);
        assert_eq!(chunk.get_block(0, 0, 0), Some(9));
        assert_eq!(chunk.get_block(11, 45, 14), Some(9));
        assert_eq!(chunk.get_block(15, 383, 15), Some(9));
        assert_eq!(chunk.get_block(0, 1, 0), Some(0));
    }
}
mod dimension {
    #[test]
    fn dimension() {
        use crate::world::dimension::Dimension;
        let mut dimension = Dimension::new(
            crate::registry::dimension_type::DIMENSION_TYPES
                .get("minecraft:overworld")
                .unwrap()
                .clone(),
            "overworld".to_string(),
        );
        dimension.set_block(1144657482, 319, -138848321, 9);
        dimension.set_block(1145, 14, 1919, 9);
        dimension.set_block(0, -64, 0, 9);
        assert_eq!(dimension.get_block(1144657482, 319, -138848321), Some(9));
        assert_eq!(dimension.get_block(1145, 14, 1919), Some(9));
        assert_eq!(dimension.get_block(0, -64, 0), Some(9));
    }
}
