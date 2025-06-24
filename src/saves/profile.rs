
use crate::lua::LuaContext;
use std::collections::HashMap;
mod joker_usage;

mod consumable_usage;
mod career_stats;
use career_stats::CareerStats;

use joker_usage::JokerUsage;
use consumable_usage::ConsumableUsage;

// Profile Table
// MEMORY  table: 0x7e12dc02a5c0
// voucher_usage   table: 0x7e12dc023250
// challenge_progress      table: 0x7e12dc0232d0
// deck_stakes     table: 0x7e12dc023210
// high_scores     table: 0x7e12dc023690
// stake   1
// deck_usage      table: 0x7e12dc023410
// career_stats    table: 0x7e12dc023190
// progress        table: 0x7e12dc023450
// hand_usage      table: 0x7e12dc0233d0

// joker_usage     table: 0x7e12dc0231d0
    //Joker Usage Table
    // j_odd_todd      table: 0x74e11c059a40
    // j_stencil       table: 0x74e11c0621e0
    // j_greedy_joker  table: 0x74e11c057510
    // j_throwback     table: 0x74e11c056a70
    // j_cry_hunger    table: 0x74e11c055620
    // j_mr_bones      table: 0x74e11c054ef0
    // j_rocket        table: 0x74e11c03fb00
    // j_cry_digitalhallucinations     table: 0x74e11c054a50
    // j_shortcut      table: 0x74e11c03eed0
    // j_crafty        table: 0x74e11c053d70
    // j_red_card      table: 0x74e11c050d80
    // j_reserved_parking      table: 0x74e11c04f610
    // j_erosion       table: 0x74e11c04fcd

        // Individual Joker Table
        // losses  table: 0x73982c05a660
            // 1       3
        // wins    table: 0x73982c05a720
            // 7       1
            // 8       2
            // 2       2
            // 3       3
            // 4       1
            // 5       1
            // 6       1
            // 1       6
        // order   52
        // count   62
        // wins_by_key     table: 0x73982c05a6c0
            // stake_gold      2
            // stake_purple    1
            // stake_black     1
            // stake_white     6
            // stake_green     3
            // stake_red       2
            // stake_blue      1
            // stake_orange    1
        // losses_by_key   table: 0x73982c05a780


// consumeable_usage       table: 0x7e12dc023290
    // Consumeable Usage Table
    // c_cry_commit    table: 0x75049406c500
    // c_hanged_man    table: 0x75049406b500
    // c_ouija table: 0x75049406cd00
    // c_cry_source    table: 0x75049406ca80
    // c_sun   table: 0x75049406c100
    // c_cry_automaton table: 0x75049406bf00
    // c_temperance    table: 0x75049406bd00
    // c_cry_ctrl_v    table: 0x75049406cf00
    // c_heirophant    table: 0x75049406ae80
    // c_talisman      table: 0x75049406b980
    // c_trance        table: 0x75049406b300
    // c_planet_x      table: 0x75049406c300
    // c_medium        table: 0x75049406a500
    // c_death table: 0x75049406b780
    // c_cry_machinecode       table: 0x75049406c200
    // c_ceres table: 0x75049406b100
    // c_cry_Klubi     table: 0x75049406ad80
    // c_cry_oboe      table: 0x75049406ad00

        // Individual Card Table
        //count   42
        //order   17
pub struct Profile {
    pub name: String,
    pub challenges_unlocked: u8,
    pub stake: u8,
    pub joker_usage: HashMap<String, JokerUsage>,
    pub consumable_usage: HashMap<String, ConsumableUsage>,
    pub career_stats: CareerStats,

}

impl Profile {
    /// Creates a new Profile
    pub fn new() -> Self {
        todo!("Placeholder");
    }


    pub fn to_lua_data(&self, lua: &LuaContext) -> Result<Vec<u8>, mlua::Error> {
        todo!("Implement Profile::to_lua_data");

    }
}