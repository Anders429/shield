#[derive(Clone, Copy, Default)]
pub(crate) struct BoundingBox {
    pub(crate) width: u8,
    pub(crate) height: u8,
}

impl BoundingBox {
    pub(crate) fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }
}
