#[derive(Debug, Clone)]
pub struct MetaItem {
    pub alerted: bool,
    pub discovered: bool,
    pub unlocked: bool,
    poss_alert: bool,
    poss_discover: bool,
    poss_unlock: bool,
}

impl MetaItem {
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
    pub fn can_be_alerted(&self) -> bool {
        self.poss_alert
    }
    pub fn can_be_discovered(&self) -> bool {
        self.poss_discover
    }
    pub fn can_be_unlocked(&self) -> bool {
        self.poss_unlock
    }
}
