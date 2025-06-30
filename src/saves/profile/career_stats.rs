pub struct CareerStats {
    // All prefixed with "c_"
    pub round_interest_cap_streak: usize,
    pub cards_played: usize,
    pub planetarium_used: usize,
    pub jokers_sold: usize,
    pub hands_played: usize,
    pub dollars_earned: f64,
    pub cards_sold: usize,
    pub wins: usize,
    pub playing_cards_bought: usize,
    pub shop_rerolls: usize,
    pub rounds: usize,
    pub tarot_reading_used: usize,
    pub losses: usize,
    pub cards_discarded: usize,
    pub single_hand_round_streak: usize,
    pub face_cards_played: usize,
    pub tarots_bought: usize,
    pub vouchers_bought: usize,
    pub shop_dollars_spent: f64,
    pub planets_bought: usize,
}

impl Default for CareerStats {
    fn default() -> Self {
        CareerStats {
            round_interest_cap_streak: 0,
            cards_played: 0,
            planetarium_used: 0,
            jokers_sold: 0,
            hands_played: 0,
            dollars_earned: 0.0,
            cards_sold: 0,
            wins: 0,
            playing_cards_bought: 0,
            shop_rerolls: 0,
            rounds: 0,
            tarot_reading_used: 0,
            losses: 0,
            cards_discarded: 0,
            single_hand_round_streak: 0,
            face_cards_played: 0,
            tarots_bought: 0,
            vouchers_bought: 0,
            shop_dollars_spent: 0.0,
            planets_bought: 0,
        }
    }
}
impl CareerStats {
    pub fn new() -> Self {
        CareerStats::default()
    }
}
