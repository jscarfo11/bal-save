#[derive(Debug, Clone)]

/// MetaItem struct for the meta struct
/// This struct is used to store the state of the items in the meta file
/// The alerted, discovered and unlocked fields are public so we can use checkboxes
pub struct MetaItem {
    /// The alerted field is used to check if the item has been alerted
    pub alerted: bool,
    /// The discovered field is used to check if the item has been discovered
    pub discovered: bool,
    /// The unlocked field is used to check if the item has been unlocked
    pub unlocked: bool,
    /// The poss_alert field is used to check if the item is alerted with 100% completion
    poss_alert: bool,
    /// The poss_discover field is used to check if the item is discovered with 100% completion
    poss_discover: bool,
    /// The poss_unlock field is used to check if the item is unlocked with 100% completion
    poss_unlock: bool,
}

impl MetaItem {
    /// Create a new MetaItem struct with the given values
    /// We determine if the item can be alerted, discovered or unlocked by checking if the value is Some
    pub fn new(
        alerted: Option<bool>,
        discovered: Option<bool>,
        unlocked: Option<bool>,
    ) -> Self {
        let poss_alert = alerted.is_some();
        let alerted = alerted.unwrap_or(false);
        let poss_discover = discovered.is_some();
        let discovered = discovered.unwrap_or(false);
        let poss_unlock = unlocked.is_some();
        let unlocked = unlocked.unwrap_or(false);

        MetaItem {
            alerted,
            discovered,
            unlocked,
            poss_alert,
            poss_discover,
            poss_unlock,
        }
    }
    /// Returns self.poss_alert
    pub fn can_be_alerted(&self) -> bool {
        self.poss_alert
    }
    /// Returns self.poss_discover
    pub fn can_be_discovered(&self) -> bool {
        self.poss_discover
    }
    /// Returns self.poss_unlock
    pub fn can_be_unlocked(&self) -> bool {
        self.poss_unlock
    }
}
