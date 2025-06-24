
use crate::lua::LuaContext;
use std::collections::HashSet;

/// A function to parse an option and return a string representation
fn option_parser(option: Option<bool>) -> String {
    match option {
        Some(true) => "Some(true)".to_string(),
        Some(false) => "Some(false)".to_string(),
        None => "None".to_string(),
    }
}

impl LuaContext {
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