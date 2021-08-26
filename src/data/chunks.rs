use crate::{chunk::{Chunk, Tilemap}, data::tiles};

pub(crate) static CHUNK: Chunk = Chunk {
    tilemap: Tilemap {
        tiles: [[Some(&tiles::GRASS); 32]; 32],
    }
};
