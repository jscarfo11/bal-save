use crate::lua::LuaContext;
use crate::saves::defaults::{ALL_META, DEFAULT_META};
mod filters;
mod metaitem;

pub use filters::Filters;
pub use metaitem::MetaItem;

use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::{HashMap, HashSet};
use std::io::Write;

/// Meta struct for the meta save file
/// This struct is used to store and modify the state of the items in the meta file
pub struct Meta {
    /// All of the items in the save file
    /// The keys are the names of the items and the values are the MetaItem structs
    items: HashMap<String, MetaItem>,
    /// The filters for the different types of items
    pub filters: Filters,
    /// The matcher for the fuzzy search
    pub matcher: SkimMatcherV2,
}

impl Meta {
    /// Create a new Meta struct with empty items
    fn new() -> Self {
        Meta {
            items: HashMap::new(),
            filters: Filters::new(),
            matcher: SkimMatcherV2::default(),
        }
    }
    /// Convert the struct into the raw data that is used in the save file
    pub fn to_lua_data(
        &self,
        lua: &LuaContext,
    ) -> Result<Vec<u8>, mlua::Error> {
        let table = lua.lua.create_table()?;
        let alerted_table = lua.lua.create_table()?;
        let discovered_table = lua.lua.create_table()?;
        let unlocked_table = lua.lua.create_table()?;

        for (name, item) in self.items.iter() {
            alerted_table.set(name.clone(), item.alerted)?;
            discovered_table.set(name.clone(), item.discovered)?;
            unlocked_table.set(name.clone(), item.unlocked)?;
        }

        table.set("alerted", alerted_table)?;
        table.set("discovered", discovered_table)?;
        table.set("unlocked", unlocked_table)?;

        // This is taken directly from the game engine of balatro, including the
        // license for the file (Not that it really matters since this is MIT licensed)
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

        let compressed = compress_func.call::<String>(table)?;
        let mut encoder = flate2::write::DeflateEncoder::new(
            Vec::new(),
            flate2::Compression::default(),
        );
        encoder.write_all(compressed.as_bytes())?;
        Ok(encoder.finish()?)
    }

    /// Convert the raw data from the save file into a Meta struct
    pub fn from_lua_table(
        lua: LuaContext,
        data: Vec<u8>,
    ) -> Result<Self, mlua::Error> {
        let table = lua.data_as_table(data, "map_table")?;
        let mut meta = Meta::from_defaults();
        let mut names: HashSet<String> = HashSet::new();

        // Access the subtables
        let alerted_table = lua.access_subtable(&table, "alerted")?;
        let discovered_table = lua.access_subtable(&table, "discovered")?;
        let unlocked_table = lua.access_subtable(&table, "unlocked")?;

        for pair in alerted_table.pairs::<String, bool>() {
            let (name, _) = pair?;
            names.insert(name.clone());
        }
        for pair in discovered_table.pairs::<String, bool>() {
            let (name, _) = pair?;
            names.insert(name.clone());
        }

        for pair in unlocked_table.pairs::<String, bool>() {
            let (name, _) = pair?;
            names.insert(name.clone());
        }

        for name in names.iter() {
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
            meta.update_item(name, alerted, discovered, unlocked);
        }

        meta.add_missing_defaults();
        Ok(meta)
    }

    /// Add the default items to the meta struct
    fn add_missing_defaults(&mut self) {
        for (name, alerted, discovered, unlocked) in DEFAULT_META.iter() {
            if !self.items.contains_key(*name) {
                self.add_item(
                    name.to_string(),
                    *alerted,
                    *discovered,
                    *unlocked,
                );
            }
        }
    }
    /// Create a new Meta struct with the default values
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

    /// Update the item in the meta struct
    /// If the item does not exist, it will be added
    fn update_item(
        &mut self,
        name: &str,
        alerted: Option<bool>,
        discovered: Option<bool>,
        unlocked: Option<bool>,
    ) {
        if self.items.contains_key(name) {
            let item = self.items.get_mut(name).unwrap(); // Safe to unwrap because we just checked if it exists
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

    // / Add an item to the meta struct
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
    /// Get a mutable reference to the item in the meta struct
    pub fn get_item(&mut self, name: &str) -> Option<&mut MetaItem> {
        self.items.get_mut(name)
    }
    /// Get a list of all the joker names in the meta struct
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

    /// Unlock all items of a certain type
    pub fn unlock_all_type(&mut self, type_: &str) {
        for (name, value) in self.items.iter_mut() {
            if name.starts_with(type_) {
                let default_value =
                    ALL_META.iter().find(|(name, _, _, _)| name == name);

                if default_value.is_none() {
                    value.alerted = true;
                    value.discovered = true;
                    value.unlocked = true;
                } else {
                    let default_value = default_value.unwrap();
                    value.alerted = default_value.1.unwrap_or(false);
                    value.discovered = default_value.2.unwrap_or(false);
                    value.unlocked = default_value.3.unwrap_or(false);
                }
            }
        }
    }
    /// Get a list of all the voucher names in the meta struct
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
    /// Get a list of all the deck names in the meta struct
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
    /// Get a list of all the card names in the meta struct
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
    /// Get a list of all the edition names in the meta struct
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
    /// Get a list of all the misc names in the meta struct
    /// This includes all items that are not cards, decks, or vouchers
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
