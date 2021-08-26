pub(crate) fn find_pixel_difference(
    player_position: u16,
    player_chunk: u8,
    screen_position: u16,
    screen_chunk: u8,
    chunk_size: u16,
) -> i16 {
    let mut position_delta = player_position as i16 - screen_position as i16;
    let mut chunk_delta = player_chunk as i8 - screen_chunk as i8;

    while chunk_delta < 0 {
        chunk_delta += 1;
        position_delta -= chunk_size as i16;
    }
    while chunk_delta > 0 {
        chunk_delta -= 1;
        position_delta += chunk_size as i16;
    }

    position_delta
}
