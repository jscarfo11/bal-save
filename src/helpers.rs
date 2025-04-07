use mlua::{Lua, Table, Value};
use std::{collections::HashMap, io::Read};

use crate::defaults::DEFAULT_META;
use std::io::Write;

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
    items: HashMap<String, MetaItem>,
}

impl Meta {
    fn new() -> Self {
        Meta {
            items: HashMap::new(),
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
        let mut meta = Meta::new();

        // Access the subtable
        let alerted_table = lua.access_subtable(&table, "alerted").unwrap();
        let discovered_table = lua.access_subtable(&table, "discovered").unwrap();
        let unlocked_table = lua.access_subtable(&table, "unlocked").unwrap();

        for pair in alerted_table.pairs::<String, bool>() {
            match pair {
                Ok((name, alerted)) => {
                    meta.add_item(name.to_string(), alerted, false, false);
                }
                Err(err) => eprintln!("Error iterating over alerted pairs: {}", err),
            }
        }

        for pair in discovered_table.pairs::<String, bool>() {
            match pair {
                Ok((name, discovered)) => {
                    if let Some(item) = meta.get_item(&name) {
                        item.discovered = discovered;
                    } else {
                        meta.add_item(name.to_string(), false, discovered, false);
                    }
                }
                Err(err) => eprintln!("Error iterating over discovered pairs: {}", err),
            }
        }

        for pair in unlocked_table.pairs::<String, bool>() {
            match pair {
                Ok((name, unlocked)) => {
                    if let Some(item) = meta.get_item(&name) {
                        item.unlocked = unlocked;
                    } else {
                        meta.add_item(name.to_string(), false, false, unlocked);
                    }
                }
                Err(err) => eprintln!("Error iterating over discovered pairs: {}", err),
            }
        }

        meta
    }
    pub fn from_defaults() -> Self {
        let mut meta = Meta::new();
        for (name, alerted, discovered, unlocked) in DEFAULT_META.iter() {
            meta.add_item(name.to_string(), *alerted, *discovered, *unlocked);
        }

        meta
    }

    fn add_item(&mut self, name: String, alerted: bool, discovered: bool, unlocked: bool) {
        let joker = MetaItem::new(alerted, discovered, unlocked);
        self.items.insert(name, joker);
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

    pub fn get_enchancement_names(&self) -> Vec<String> {
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

    // pub fn get_enchancement_names(&self) -> Vec<String> {
    //     self.enchancements.keys().cloned().collect()
    // }
}

#[derive(Debug, Clone)]
pub struct MetaItem {
    pub alerted: bool,
    pub discovered: bool,
    pub unlocked: bool,
}

impl MetaItem {
    fn new(alerted: bool, discovered: bool, unlocked: bool) -> Self {
        MetaItem {
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

pub struct Popup {
    show: bool,
    message: String,
    button: String,
}

impl Popup {
    pub fn new(msg: &str, btn: &str) -> Self {
        Popup {
            show: true,
            message: msg.to_string(),
            button: btn.to_string(),
        }
    }
    pub fn show(&mut self) {
        self.show = true;
    }
    pub fn hide(&mut self) {
        self.show = false;
    }
    pub fn is_showing(&self) -> bool {
        self.show
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn get_button(&self) -> &str {
        &self.button
    }
}
