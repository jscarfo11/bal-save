#[derive(Debug, Clone)]
pub enum TabState {
    Editor,
    Settings,
    Help,
    None,
}

impl PartialEq for TabState {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}
impl Eq for TabState {}