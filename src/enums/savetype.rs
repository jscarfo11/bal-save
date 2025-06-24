use crate::saves::Meta;
use crate::saves::Profile;

pub enum SaveType {
    /// Represents a Meta save type.
    Meta(Meta),
    // /// Represents a Profile save type.
    Profile(Profile),
    // /// Represents a Savegame save type.
    // Save(String)
}

impl From<SaveType> for Meta {
    fn from(save_type: SaveType) -> Self {
        match save_type {
            SaveType::Meta(meta) => meta,
            _ => Meta::from_defaults(),
        }
    }
}

// impl From<SaveType> for Profile {
//     fn from(save_type: SaveType) -> Self {
//         match save_type {
//             SaveType::Profile(profile) => profile,
//             _ => panic!("Cannot convert to Profile from non-Profile SaveType"),
//         }
//     }
// }
