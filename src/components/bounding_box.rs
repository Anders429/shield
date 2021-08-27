#[derive(Clone, Copy, Default)]
pub(crate) struct BoundingBox {
    pub(crate) width: u8,
    pub(crate) height: u8,
    pub(crate) offset_x: u8,
    pub(crate) offset_y: u8,
}
