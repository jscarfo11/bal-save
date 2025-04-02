use mlua::{Lua, Table, Value};
use std::{collections::HashMap, io::Read};

use crate::defaults::{CARDS, DECKS, ENCHANCEMENTS, JOKERS, VOUCHERS};

pub struct LuaContext {
    lua: Lua,
}

impl LuaContext {
    pub fn new() -> Self {
        LuaContext { lua: Lua::new() }
    }

    pub fn data_as_table(&self, data: Vec<u8>, tablename: &str) -> Result<Table, mlua::Error> {
        let mut decoder = flate2::read::DeflateDecoder::new(&data[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        let lua = &self.lua;
        let val: Table = lua.load(&s).eval()?;
        lua.globals().set(tablename, &val)?;
        Ok(val)
    }

    pub fn access_subtable(&self, val: &Table, subtable_name: &str) -> Result<Table, mlua::Error> {
        if let Some(Value::Table(subtable)) = val.get(subtable_name.to_string())? {
            return Ok(subtable);
        }

        Err(mlua::Error::RuntimeError(format!(
            "Subtable '{}' not found or not a table",
            subtable_name
        )))
    }

    pub fn clone(&self) -> Self {
        LuaContext {
            lua: self.lua.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    jokers: HashMap<String, Joker>,
    vouchers: HashMap<String, Voucher>,
    decks: HashMap<String, Deck>,
    cards: HashMap<String, Card>,
    enchancements: HashMap<String, Enchancement>,
}

impl Meta {
    fn new() -> Self {
        Meta {
            jokers: HashMap::new(),
            vouchers: HashMap::new(),
            decks: HashMap::new(),
            cards: HashMap::new(),
            enchancements: HashMap::new(),
        }
    }

    pub fn from_lua_table(lua: LuaContext, data: Vec<u8>) -> Self {
        let table = lua.data_as_table(data, "map_table").unwrap();
        let mut meta = Meta::new();

        // Access the subtable
        let alerted_table = lua.access_subtable(&table, "alerted").unwrap();
        let discovered_table = lua.access_subtable(&table, "discovered").unwrap();
        let unlocked_table = lua.access_subtable(&table, "unlocked").unwrap();

        for (pair) in alerted_table.pairs::<String, bool>() {
            match pair {
                Ok((name, alerted)) => {
                    if name.starts_with("j_") {
                        meta.add_joker(name.to_string(), alerted, false, false);
                    } else if name.starts_with("v_") {
                        meta.add_voucher(name.to_string(), alerted, false, false);
                    } else if name.starts_with("b_") {
                        meta.add_deck(name.to_string(), alerted, false, false);
                    } else if name.starts_with("c_") {
                        meta.add_card(name.to_string(), alerted, false, false);
                    } else if name.starts_with("e_") {
                        meta.add_enchancement(name.to_string(), alerted, false, false);
                    }
                }
                Err(err) => eprintln!("Error iterating over alerted pairs: {}", err),
            }
        }

        for pair in discovered_table.pairs::<String, bool>() {
            match pair {
                Ok((name, discovered)) => {
                    if name.starts_with("j_") {
                        if let Some(joker) = meta.get_joker(&name) {
                            joker.discovered = discovered;
                        } else {
                            meta.add_joker(name.to_string(), false, discovered, false);
                        }
                    } else if name.starts_with("v_") {
                        if let Some(voucher) = meta.get_voucher(&name) {
                            voucher.discovered = discovered;
                        } else {
                            meta.add_voucher(name.to_string(), false, discovered, false);
                        }
                    } else if name.starts_with("b_") {
                        if let Some(deck) = meta.get_deck(&name) {
                            deck.discovered = discovered;
                        } else {
                            meta.add_deck(name.to_string(), false, discovered, false);
                        }
                    } else if name.starts_with("c_") {
                        if let Some(card) = meta.get_card(&name) {
                            card.discovered = discovered;
                        } else {
                            meta.add_card(name.to_string(), false, discovered, false);
                        }
                    } else if name.starts_with("e_") {
                        if let Some(enchancement) = meta.get_enchancement(&name) {
                            enchancement.discovered = discovered;
                        } else {
                            meta.add_enchancement(name.to_string(), false, discovered, false);
                        }
                    }
                }
                Err(err) => eprintln!("Error iterating over discovered pairs: {}", err),
            }
        }

        for pair in unlocked_table.pairs::<String, bool>() {
            match pair {
                Ok((name, unlocked)) => {
                    if name.starts_with("j_") {
                        if let Some(joker) = meta.get_joker(&name) {
                            joker.unlocked = unlocked;
                        } else {
                            meta.add_joker(name.to_string(), false, false, unlocked);
                        }
                    } else if name.starts_with("v_") {
                        if let Some(voucher) = meta.get_voucher(&name) {
                            voucher.unlocked = unlocked;
                        } else {
                            meta.add_voucher(name.to_string(), false, false, unlocked);
                        }
                    } else if name.starts_with("b_") {
                        if let Some(deck) = meta.get_deck(&name) {
                            deck.unlocked = unlocked;
                        } else {
                            meta.add_deck(name.to_string(), false, false, unlocked);
                        }
                    } else if name.starts_with("c_") {
                        if let Some(card) = meta.get_card(&name) {
                            card.unlocked = unlocked;
                        } else {
                            meta.add_card(name.to_string(), false, false, unlocked);
                        }
                    } else if name.starts_with("e_") {
                        if let Some(enchancement) = meta.get_enchancement(&name) {
                            enchancement.unlocked = unlocked;
                        } else {
                            meta.add_enchancement(name.to_string(), false, false, unlocked);
                        }
                    }
                }
                Err(err) => eprintln!("Error iterating over discovered pairs: {}", err),
            }
        }

        meta
    }
    pub fn from_defaults() -> Self {
        let mut meta = Meta::new();
        for (name, alerted, discovered, unlocked) in JOKERS.iter() {
            meta.add_joker(name.to_string(), *alerted, *discovered, *unlocked);
        }
        for (name, alerted, discovered, unlocked) in VOUCHERS.iter() {
            meta.add_voucher(name.to_string(), *alerted, *discovered, *unlocked);
        }
        for (name, alerted, discovered, unlocked) in DECKS.iter() {
            meta.add_deck(name.to_string(), *alerted, *discovered, *unlocked);
        }

        for (name, alerted, discovered, unlocked) in CARDS.iter() {
            meta.add_card(name.to_string(), *alerted, *discovered, *unlocked);
        }
        for (name, alerted, discovered, unlocked) in ENCHANCEMENTS.iter() {
            meta.add_enchancement(name.to_string(), *alerted, *discovered, *unlocked);
        }

        meta
    }

    fn add_joker(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let joker = Joker::new(name.clone(), alerted, discovered, unlocked);
        self.jokers.insert(name, joker);
    }
    pub fn get_joker(&mut self, name: &str) -> Option<&mut Joker> {
        self.jokers.get_mut(name)
    }
    fn get_all_jokers(&self) -> Vec<&Joker> {
        self.jokers.values().collect()
    }
    pub fn get_joker_names(&self) -> Vec<String> {
        self.jokers.keys().cloned().collect()
    }

    pub fn unlock_all_jokers(&mut self) {
        for joker in self.jokers.values_mut() {
            joker.unlocked = true;
            joker.alerted = true;
            joker.discovered = true;
        }
    }

    fn add_voucher(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let voucher = Voucher::new(name.clone(), alerted, discovered, unlocked);
        self.vouchers.insert(name, voucher);
    }

    pub fn get_voucher(&mut self, name: &str) -> Option<&mut Voucher> {
        self.vouchers.get_mut(name)
    }
    fn get_all_vouchers(&self) -> Vec<&Voucher> {
        self.vouchers.values().collect()
    }
    pub fn get_voucher_names(&self) -> Vec<String> {
        self.vouchers.keys().cloned().collect()
    }

    pub fn unlock_all_vouchers(&mut self) {
        for voucher in self.vouchers.values_mut() {
            voucher.unlocked = true;
            voucher.alerted = true;
            voucher.discovered = true;
        }
    }

    fn get_all_decks(&self) -> Vec<&Deck> {
        self.decks.values().collect()
    }
    fn get_deck_names(&self) -> Vec<String> {
        self.decks.keys().cloned().collect()
    }
    fn add_deck(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let deck = Deck::new(name.clone(), alerted, discovered, unlocked);
        self.decks.insert(name, deck);
    }
    fn get_deck(&mut self, name: &str) -> Option<&mut Deck> {
        self.decks.get_mut(name)
    }

    pub fn unlock_all_decks(&mut self) {
        for deck in self.decks.values_mut() {
            deck.unlocked = true;
            deck.alerted = true;
            deck.discovered = true;
        }
    }

    fn add_card(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let card = Card::new(name.clone(), alerted, discovered, unlocked);
        self.cards.insert(name, card);
    }
    fn get_card(&mut self, name: &str) -> Option<&mut Card> {
        self.cards.get_mut(name)
    }
    fn get_all_cards(&self) -> Vec<&Card> {
        self.cards.values().collect()
    }
    fn get_card_names(&self) -> Vec<String> {
        self.cards.keys().cloned().collect()
    }

    pub fn unlock_all_cards(&mut self) {
        for card in self.cards.values_mut() {
            card.unlocked = true;
            card.alerted = true;
            card.discovered = true;
        }
    }

    fn add_enchancement(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let enchancement = Enchancement::new(name.clone(), alerted, discovered, unlocked);
        self.enchancements.insert(name, enchancement);
    }
    pub fn get_enchancement(&mut self, name: &str) -> Option<&mut Enchancement> {
        self.enchancements.get_mut(name)
    }
    fn get_all_enchancements(&self) -> Vec<&Enchancement> {
        self.enchancements.values().collect()
    }
    pub fn get_enchancement_names(&self) -> Vec<String> {
        self.enchancements.keys().cloned().collect()
    }

    pub fn unlock_all_enchancements(&mut self) {
        for enchancement in self.enchancements.values_mut() {
            enchancement.unlocked = true;
            enchancement.alerted = true;
            enchancement.discovered = true;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Joker {
    name: String,
    pub alerted: bool,
    pub discovered: bool,
    pub unlocked: bool,
}

impl Joker {
    fn new(name: String, alerted: bool, discovered: bool, unlocked: bool) -> Self {
        Joker {
            name,
            alerted,
            discovered,
            unlocked,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Voucher {
    name: String,
    pub alerted: bool,
    pub discovered: bool,
    pub unlocked: bool,
}
impl Voucher {
    fn new(name: String, alerted: bool, discovered: bool, unlocked: bool) -> Self {
        Voucher {
            name,
            alerted,
            discovered,
            unlocked,
        }
    }
}

#[derive(Debug, Clone)]
struct Deck {
    name: String,
    alerted: bool,
    discovered: bool,
    unlocked: bool,
}
impl Deck {
    fn new(name: String, alerted: bool, discovered: bool, unlocked: bool) -> Self {
        Deck {
            name,
            alerted,
            discovered,
            unlocked,
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    name: String,
    alerted: bool,
    discovered: bool,
    unlocked: bool,
}
impl Card {
    fn new(name: String, alerted: bool, discovered: bool, unlocked: bool) -> Self {
        Card {
            name,
            alerted,
            discovered,
            unlocked,
        }
    }
}

#[derive(Debug, Clone)]
struct Enchancement {
    name: String,
    alerted: bool,
    discovered: bool,
    unlocked: bool,
}
impl Enchancement {
    fn new(name: String, alerted: bool, discovered: bool, unlocked: bool) -> Self {
        Enchancement {
            name,
            alerted,
            discovered,
            unlocked,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TabState {
    Editor,
    Settings,
    Help,
    None,
}

impl PartialEq for TabState {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}
impl Eq for TabState {}
