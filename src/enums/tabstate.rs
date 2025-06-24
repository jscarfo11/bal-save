#[derive(Debug, Clone)]
///enum for the different tabs in the app
pub enum TabState {
    /// The tab for the Editor
    Editor,
    /// The tab for the Settings
    Settings,
    /// The tab for Help
    Help,
    /// The default tab
    None,
    // The tab for dev crap
    Dev
}
/// Lets you check two TabStates are equal
impl PartialEq for TabState {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}
impl Eq for TabState {}
