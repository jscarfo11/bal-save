#[derive(Debug, Clone)]
/// Enum for the different types of popups in the app
pub enum PopupType {
    /// The popup for an error when saving a file
    ErrorSave,
    /// The popup for an error when loading a file
    ErrorLoad,
    /// The popup for confirming overwritting the current meta with defaults
    ConfirmMetaDefault,
    /// The popup for confirming overwritting the current meta with a new file
    ConfirmMetaFile,
}
