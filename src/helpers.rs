
use mlua::{Lua, Table, Value};
use std::{collections::HashMap, io::Read};

use crate::defaults::{JOKERS, VOUCHERS, DECKS, CARDS, ENCHANCEMENTS};


pub struct LuaContext {
    lua: Lua,
}

impl LuaContext {
    pub fn new() -> Self {
        LuaContext {
            lua: Lua::new(),
        }
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
    fn from_defaults() -> Self {
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
    fn get_joker(&self, name: &str) -> Option<&Joker> {
        self.jokers.get(name)
    }
    fn get_all_jokers(&self) -> Vec<&Joker> {
        self.jokers.values().collect()
    }
    fn get_joker_names(&self) -> Vec<String> {
        self.jokers.keys().cloned().collect()
    }

    fn add_voucher(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let voucher = Voucher::new(name.clone(), alerted, discovered, unlocked);
        self.vouchers.insert(name, voucher);
    }

    fn get_voucher(&self, name: &str) -> Option<&Voucher> {
        self.vouchers.get(name)
    }
    fn get_all_vouchers(&self) -> Vec<&Voucher> {
        self.vouchers.values().collect()
    }
    fn get_voucher_names(&self) -> Vec<String> {
        self.vouchers.keys().cloned().collect()
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
    fn get_deck(&self, name: &str) -> Option<&Deck> {
        self.decks.get(name)
    }


    fn add_card(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let card = Card::new(name.clone(), alerted, discovered, unlocked);
        self.cards.insert(name, card);
    }
    fn get_card(&self, name: &str) -> Option<&Card> {
        self.cards.get(name)
    }
    fn get_all_cards(&self) -> Vec<&Card> {
        self.cards.values().collect()
    }
    fn get_card_names(&self) -> Vec<String> {
        self.cards.keys().cloned().collect()
    }
    fn add_enchancement(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let enchancement = Enchancement::new(name.clone(), alerted, discovered, unlocked);
        self.enchancements.insert(name, enchancement);
    }
    fn get_enchancement(&self, name: &str) -> Option<&Enchancement> {
        self.enchancements.get(name)
    }
    fn get_all_enchancements(&self) -> Vec<&Enchancement> {
        self.enchancements.values().collect()
    }
    fn get_enchancement_names(&self) -> Vec<String> {
        self.enchancements.keys().cloned().collect()
    }




        

}




struct Joker {
    name: String,
    alerted: bool,
    discovered: bool,
    unlocked: bool,
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


struct Voucher {
    name: String,
    alerted: bool,
    discovered: bool,
    unlocked: bool,
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
