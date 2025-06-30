use crate::lua::LuaContext;
use std::sync::mpsc::{Receiver, Sender, channel};

#[cfg(feature = "dev")]
pub struct DevTest {
    pub lua: LuaContext,
    pub table: Option<mlua::Table>,
    pub save_data: Vec<u8>,
    pub data_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    pub lua_string: String,
    pub output: String,
}
#[cfg(feature = "dev")]
impl DevTest {
    /// Creates a new DevTest
    pub fn new() -> Self {
        DevTest {
            lua: LuaContext::new(),
            table: None,
            save_data: vec![],
            data_channel: channel(),
            lua_string: String::new(),
            output: String::new(),
        }
    }
}
