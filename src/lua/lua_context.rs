use mlua::{Lua, Table, Value};
use std::collections::HashSet;
use std::io::Read;

pub struct LuaContext {
    pub lua: Lua,
}

fn option_parser(option: Option<bool>) -> String {
    match option {
        Some(true) => "Some(true)".to_string(),
        Some(false) => "Some(false)".to_string(),
        None => "None".to_string(),
    }
}

impl LuaContext {
    pub fn new() -> Self {
        LuaContext { lua: Lua::new() }
    }

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

    pub fn clone(&self) -> Self {
        LuaContext { lua: self.lua.clone() }
    }

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
