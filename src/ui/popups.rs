use crate::enums::PopupType;
#[derive(Debug, Clone)]
pub struct Popup {
    popup_type: PopupType,
    message: String,
}

impl Popup {
    pub fn new(popup_type: PopupType, message: String) -> Self {
        Popup {
            popup_type,
            message,

        }
    }
    pub fn get_type(&self) -> PopupType {
        self.popup_type.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}