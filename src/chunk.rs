use crate::{constants, tile::Tile};

pub(crate) struct Tilemap<'a> {
    pub(crate) tiles:
        [[Option<&'a Tile<'a>>; constants::CHUNK_TILE_WIDTH]; constants::CHUNK_TILE_HEIGHT],
}

pub(crate) struct Chunk<'a> {
    pub(crate) layer_1: Tilemap<'a>,
    pub(crate) layer_2: Tilemap<'a>,
}
