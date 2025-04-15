use mlua::{Lua, Table, Value};
use std::collections::{HashMap, HashSet};

use crate::defaults::{ALL_META, DEFAULT_META};
use std::io::{Read, Write};

pub struct LuaContext {
    lua: Lua,
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

    pub fn make_meta_defaults(&self, data: Vec<u8>) -> Result<(), mlua::Error> {
        let t = self.data_as_table(data, "map_table")?;

        let alerted_table = self.access_subtable(&t, "alerted")?;
        let discovered_table = self.access_subtable(&t, "discovered")?;
        let unlocked_table = self.access_subtable(&t, "unlocked")?;
        let mut set = HashSet::new();

        for pair in alerted_table.pairs::<String, bool>() {
            match pair {
                Ok((name, _)) => {
                    if name.contains("cry_") || name.contains("mp_") || name.contains("mtg_") {
                        continue;
                    }
                    set.insert(name);
                }
                Err(err) => eprintln!("Error iterating over alerted pairs: {}", err),
            }
        }

        for pair in discovered_table.pairs::<String, bool>() {
            match pair {
                Ok((name, _)) => {
                    if name.contains("cry_") || name.contains("mp_") || name.contains("mtg_") {
                        continue;
                    }
                    set.insert(name);
                }
                Err(err) => eprintln!("Error iterating over discovered pairs: {}", err),
            }
        }

        for pair in unlocked_table.pairs::<String, bool>() {
            match pair {
                Ok((name, _)) => {
                    if name.contains("cry_") || name.contains("mp_") || name.contains("mtg_") {
                        continue;
                    }
                    set.insert(name);
                }
                Err(err) => eprintln!("Error iterating over discovered pairs: {}", err),
            }
        }

        for name in set.iter() {
            let alerted: Option<bool> = if alerted_table.contains_key(name.clone())? {
                Some(alerted_table.get(name.clone())?)
            } else {
                None
            };

            let discovered: Option<bool> = if discovered_table.contains_key(name.clone())? {
                Some(discovered_table.get(name.clone())?)
            } else {
                None
            };

            let unlocked: Option<bool> = if unlocked_table.contains_key(name.clone())? {
                Some(unlocked_table.get(name.clone())?)
            } else {
                None
            };
            let alerted = option_parser(alerted);
            let discovered = option_parser(discovered);
            let unlocked = option_parser(unlocked);

            println!("(\"{}\", {}, {}, {}),", name, alerted, discovered, unlocked);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    items: HashMap<String, MetaItem>,
    pub filters: Filters,
}

impl Meta {
    fn new() -> Self {
        Meta {
            items: HashMap::new(),
            filters: Filters::new(),
        }
    }

    pub fn to_lua_data(&self, lua: &LuaContext) -> Result<Vec<u8>, mlua::Error> {
        let table = lua.lua.create_table().unwrap();
        let alerted_table = lua.lua.create_table().unwrap();
        let discovered_table = lua.lua.create_table().unwrap();
        let unlocked_table = lua.lua.create_table().unwrap();

        for (name, item) in self.items.iter() {
            alerted_table.set(name.clone(), item.alerted).unwrap();
            discovered_table.set(name.clone(), item.discovered).unwrap();
            unlocked_table.set(name.clone(), item.unlocked).unwrap();
        }

        table
            .set("alerted", alerted_table)
            .expect("Failed to set alerted table");
        table
            .set("discovered", discovered_table)
            .expect("Failed to set discovered table");
        table
            .set("unlocked", unlocked_table)
            .expect("Failed to set unlocked table");

        let compress_func: mlua::Function= lua.lua.load(
            r#"
            --[[
MIT License
Copyright (c) 2017 Robert Herlihy
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
]]

--I modified this A LOT. Needed to make it quicker if it is being saved to file every few seconds during a game
function STR_PACK(data, recursive)
	local ret_str = (recursive and "" or "return ").."{"

      for i, v in pairs(data) do
		local type_i, type_v = type(i), type(v)
        assert((type_i ~= "table"), "Data table cannot have an table as a key reference")
        if type_i == "string" then
			i = '['..string.format("%q",i)..']'
        else
          	i = "["..i.."]"
        end
        if type_v == "table" then
			if v.is and v:is(Object) then
				v = [["]].."MANUAL_REPLACE"..[["]]
			else
				v = STR_PACK(v, true)
			end
        else
          if type_v == "string" then v = string.format("%q", v) end
		  if type_v == "boolean" then v = v and "true" or "false" end
        end
		ret_str = ret_str..i.."="..v..","
      end

	  return ret_str.."}"

end

return STR_PACK
"#
        ).eval()?;

        let compressed = compress_func.call::<String>(table).unwrap();
        let mut encoder =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(compressed.as_bytes()).unwrap();
        Ok(encoder.finish().unwrap())
    }

    pub fn from_lua_table(lua: LuaContext, data: Vec<u8>) -> Self {
        let table = lua.data_as_table(data, "map_table").unwrap();
        let mut meta = Meta::from_defaults();
        let mut names: HashSet<String> = HashSet::new();

        // Access the subtable
        let alerted_table = lua.access_subtable(&table, "alerted").unwrap();
        let discovered_table = lua.access_subtable(&table, "discovered").unwrap();
        let unlocked_table = lua.access_subtable(&table, "unlocked").unwrap();

        for pair in alerted_table.pairs::<String, bool>() {
            let (name, _) = pair.unwrap();
            names.insert(name.clone());
        }
        for pair in discovered_table.pairs::<String, bool>() {
            let (name, _) = pair.unwrap();
            names.insert(name.clone());
        }

        for pair in unlocked_table.pairs::<String, bool>() {
            let (name, _) = pair.unwrap();
            names.insert(name.clone());
        }

        for name in names.iter() {
            let alerted: Option<bool> = if alerted_table.contains_key(name.clone()).unwrap() {
                Some(alerted_table.get(name.clone()).unwrap())
            } else {
                None
            };

            let discovered: Option<bool> = if discovered_table.contains_key(name.clone()).unwrap() {
                Some(discovered_table.get(name.clone()).unwrap())
            } else {
                None
            };

            let unlocked: Option<bool> = if unlocked_table.contains_key(name.clone()).unwrap() {
                Some(unlocked_table.get(name.clone()).unwrap())
            } else {
                None
            };
            meta.update_item(name, alerted, discovered, unlocked);
        }

        meta.add_missing_defaults();
        meta
    }

    fn add_missing_defaults(&mut self) {
        for (name, alerted, discovered, unlocked) in DEFAULT_META.iter() {
            if !self.items.contains_key(*name) {
                self.add_item(name.to_string(), *alerted, *discovered, *unlocked);
            }
        }
    }
    pub fn from_defaults() -> Self {
        let mut meta = Meta::new();
        for (name, alerted, discovered, unlocked) in ALL_META.iter() {
            let mut alerted = *alerted;
            let mut discovered = *discovered;
            let mut unlocked = *unlocked;

            if alerted.is_some() {
                alerted = Some(false);
            }
            if discovered.is_some() {
                discovered = Some(false);
            }
            if unlocked.is_some() {
                unlocked = Some(false);
            }

            meta.add_item(name.to_string(), alerted, discovered, unlocked);
        }

        for (name, alerted, discovered, unlocked) in DEFAULT_META.iter() {
            meta.update_item(*name, *alerted, *discovered, *unlocked);
        }

        meta
    }

    fn update_item(
        &mut self,
        name: &str,
        alerted: Option<bool>,
        discovered: Option<bool>,
        unlocked: Option<bool>,
    ) {
        if self.items.contains_key(name) {
            let item = self.items.get_mut(name).unwrap();
            item.alerted = alerted.unwrap_or(false);
            item.discovered = discovered.unwrap_or(false);
            item.unlocked = unlocked.unwrap_or(false);
        } else {
            // This means we have a modded item so we cannot be sure about what should be modified, so we let everything be modified
            let mut alerted = alerted;
            let mut discovered = discovered;
            let mut unlocked = unlocked;

            if alerted.is_none() {
                alerted = Some(false);
            }
            if discovered.is_none() {
                discovered = Some(false);
            }
            if unlocked.is_none() {
                unlocked = Some(false);
            }
            let item = MetaItem::new(alerted, discovered, unlocked);
            self.items.insert(name.to_string(), item);
        }
    }

    fn add_item(
        &mut self,
        name: String,
        alerted: Option<bool>,
        discovered: Option<bool>,
        unlocked: Option<bool>,
    ) {
        let item = MetaItem::new(alerted, discovered, unlocked);
        self.items.insert(name, item);
    }
    pub fn get_item(&mut self, name: &str) -> Option<&mut MetaItem> {
        self.items.get_mut(name)
    }

    pub fn get_joker_names(&self) -> Vec<String> {
        let mut all: Vec<String> = self.items.keys().cloned().collect();
        all.retain_mut(|name| {
            if name.starts_with("j_") {
                return true;
            }
            false
        });
        all.sort();
        all
    }

    pub fn unlock_all_type(&mut self, type_: &str) {
        for (name, value) in self.items.iter_mut() {
            if name.starts_with(type_) {
                value.unlocked = true;
                value.alerted = true;
                value.discovered = true;
            }
        }
    }

    pub fn get_voucher_names(&self) -> Vec<String> {
        let mut all: Vec<String> = self.items.keys().cloned().collect();
        all.retain_mut(|name| {
            if name.starts_with("v_") {
                return true;
            }
            false
        });
        all.sort();
        all
    }

    pub fn get_deck_names(&self) -> Vec<String> {
        let mut all: Vec<String> = self.items.keys().cloned().collect();
        all.retain_mut(|name| {
            if name.starts_with("b_") {
                return true;
            }
            false
        });
        all.sort();
        all
    }
    pub fn get_card_names(&self) -> Vec<String> {
        let mut all: Vec<String> = self.items.keys().cloned().collect();
        all.retain_mut(|name| {
            if name.starts_with("c_") {
                return true;
            }
            false
        });
        all.sort();
        all
    }

    pub fn get_edition_names(&self) -> Vec<String> {
        let mut all: Vec<String> = self.items.keys().cloned().collect();
        all.retain_mut(|name| {
            if name.starts_with("e_") {
                return true;
            }
            false
        });
        all.sort();
        all
    }

    pub fn get_misc_names(&self) -> Vec<String> {
        let mut all: Vec<String> = self.items.keys().cloned().collect();
        let starting: [&str; 5] = ["b_", "e_", "bl_", "tag_", "p_"];
        all.retain_mut(|name| {
            for start in starting.iter() {
                if name.starts_with(*start) {
                    return true;
                }
            }

            false
        });
        all.sort();
        all
    }
}

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
    fn new(alerted: Option<bool>, discovered: Option<bool>, unlocked: Option<bool>) -> Self {
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

#[derive(Debug, Clone)]
pub enum PopupType {
    ErrorSave,
    ErrorLoad,
}

pub struct Popup {
    popup_type: PopupType,
    message: String,
}

impl Popup {
    pub fn new(popup_type: PopupType, message: String) -> Self {
        Popup {
            popup_type,
            message,

        }
    }
    pub fn get_type(&self) -> PopupType {
        self.popup_type.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}


#[derive(Debug, Clone)]
pub struct Filters {
    pub joker: String,
    pub misc: String,
    pub card: String,

    pub voucher: String,
}

impl Filters {
    pub fn new() -> Self {
        Filters {
            joker: String::new(),
            misc: String::new(),
            card: String::new(),
            voucher: String::new(),
        }
    }
}
