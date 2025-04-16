use crate::enums::PopupType;
#[derive(Debug, Clone)]

/// Popup struct to represent a popup message in the application
pub struct Popup {
    /// Classification of the popup
    popup_type: PopupType,
    /// Message to be displayed, if any
    message: String,
}

impl Popup {
    /// Creates a new Popup instance
    ///
    /// # Arguments
    /// popup_type: The type of the popup
    /// message: The message to be displayed in the popup
    pub fn new(popup_type: PopupType, message: String) -> Self {
        Popup { popup_type, message }
    }

    // Returns the type of the popup
    pub fn get_type(&self) -> PopupType {
        self.popup_type.clone()
    }
    // Returns the message of the popup
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
