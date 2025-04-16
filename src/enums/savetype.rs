
use crate::saves::Meta;


pub enum SaveType {
    /// Represents a Meta save type.
    Meta(Meta),
    // /// Represents a Profile save type.
    // Profile(String),
    // /// Represents a Savegame save type.
    // Save(String)
}

impl From<SaveType> for Meta {
    fn from(save_type: SaveType) -> Self {
        match save_type {
            SaveType::Meta(meta) => meta,
            
        }
    }
}