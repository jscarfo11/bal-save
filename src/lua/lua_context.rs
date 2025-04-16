use mlua::{Lua, Table, Value};
use std::collections::HashSet;
use std::io::Read;

/// A struct to hold the lua LVM
/// It is used because all lua tables only exists in their own lvm, so it allows for a
/// nice place to put all relevant lua functions
pub struct LuaContext {
    pub lua: Lua,
}

/// A function to parse an option and return a string representation
fn option_parser(option: Option<bool>) -> String {
    match option {
        Some(true) => "Some(true)".to_string(),
        Some(false) => "Some(false)".to_string(),
        None => "None".to_string(),
    }
}

impl LuaContext {
    /// Creates a new LuaContext
    pub fn new() -> Self {
        LuaContext { lua: Lua::new() }
    }
    /// Loads a balatro save file into a lua table
    pub fn data_as_table(
        &self,
        data: Vec<u8>,
        tablename: &str,
    ) -> Result<Table, mlua::Error> {
        let mut decoder = flate2::read::DeflateDecoder::new(&data[..]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;
        let lua = &self.lua;
        let val: Table = lua.load(&s).eval()?;
        lua.globals().set(tablename, &val)?;
        Ok(val)
    }
    /// Accesses a subtable of a table
    pub fn access_subtable(
        &self,
        val: &Table,
        subtable_name: &str,
    ) -> Result<Table, mlua::Error> {
        if let Some(Value::Table(subtable)) =
            val.get(subtable_name.to_string())?
        {
            return Ok(subtable);
        }

        Err(mlua::Error::RuntimeError(format!(
            "Subtable '{}' not found or not a table",
            subtable_name
        )))
    }

    /// Print out the contents of a save file in the meta default format
    pub fn make_meta_defaults(&self, data: Vec<u8>) -> Result<(), mlua::Error> {
        let t = self.data_as_table(data, "map_table")?;

        let alerted_table = self.access_subtable(&t, "alerted")?;
        let discovered_table = self.access_subtable(&t, "discovered")?;
        let unlocked_table = self.access_subtable(&t, "unlocked")?;
        let mut set = HashSet::new();

        for pair in alerted_table.pairs::<String, bool>() {
            match pair {
                Ok((name, _)) => {
                    if name.contains("cry_")
                        || name.contains("mp_")
                        || name.contains("mtg_")
                    {
                        continue;
                    }
                    set.insert(name);
                }
                Err(err) => {
                    eprintln!("Error iterating over alerted pairs: {}", err)
                }
            }
        }

        for pair in discovered_table.pairs::<String, bool>() {
            match pair {
                Ok((name, _)) => {
                    if name.contains("cry_")
                        || name.contains("mp_")
                        || name.contains("mtg_")
                    {
                        continue;
                    }
                    set.insert(name);
                }
                Err(err) => {
                    eprintln!("Error iterating over discovered pairs: {}", err)
                }
            }
        }

        for pair in unlocked_table.pairs::<String, bool>() {
            match pair {
                Ok((name, _)) => {
                    if name.contains("cry_")
                        || name.contains("mp_")
                        || name.contains("mtg_")
                    {
                        continue;
                    }
                    set.insert(name);
                }
                Err(err) => {
                    eprintln!("Error iterating over discovered pairs: {}", err)
                }
            }
        }

        for name in set.iter() {
            let alerted: Option<bool> =
                if alerted_table.contains_key(name.clone())? {
                    Some(alerted_table.get(name.clone())?)
                } else {
                    None
                };

            let discovered: Option<bool> =
                if discovered_table.contains_key(name.clone())? {
                    Some(discovered_table.get(name.clone())?)
                } else {
                    None
                };

            let unlocked: Option<bool> =
                if unlocked_table.contains_key(name.clone())? {
                    Some(unlocked_table.get(name.clone())?)
                } else {
                    None
                };
            let alerted = option_parser(alerted);
            let discovered = option_parser(discovered);
            let unlocked = option_parser(unlocked);

            println!(
                "(\"{}\", {}, {}, {}),",
                name, alerted, discovered, unlocked
            );
        }

        Ok(())
    }
}

// Meta Table
// alerted table: 0x7b9cbc005610
// discovered      table: 0x7b9cbc003f60
// unlocked        table: 0x7b9cbc003f20
// Table(Ref(0x7b9cbc0286f0))

// Profile Table
// MEMORY  table: 0x7e12dc02a5c0
// consumeable_usage       table: 0x7e12dc023290
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
// Table(Ref(0x7e12dc023150))

// Save Table
// STATE   5
// BLIND   table: 0x75142c059620
// tags    table: 0x75142c059ae0
// VERSION 1.0.1o-FULL
// GAME    table: 0x75142c0a8ce0
// BACK    table: 0x75142c0596a0
// cardAreas       table: 0x75142c059b20
// Table(Ref(0x75142c0595e0))
