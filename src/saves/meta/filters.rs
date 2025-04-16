#[derive(Debug, Clone)]

/// Filters for the different types of items in the meta file
/// All the fields are public as they are used as references in the egui App
pub struct Filters {
    /// Filter for joker items
    pub joker: String,
    /// Filter for misc items
    pub misc: String,
    /// Filter for card items
    pub card: String,
    /// Filter for voucher items
    pub voucher: String,
}

impl Filters {
    /// Create a new Filters struct with empty strings
    pub fn new() -> Self {
        Filters {
            joker: String::new(),
            misc: String::new(),
            card: String::new(),
            voucher: String::new(),
        }
    }
}
