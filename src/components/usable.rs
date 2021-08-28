#[derive(Clone, Copy, Debug)]
pub(crate) enum Usable {
    None,
    Shield,
}

impl Default for Usable {
    fn default() -> Self {
        Self::None
    }
}
