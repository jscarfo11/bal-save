
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::lua::LuaContext;




pub struct DevTest {
    pub lua: LuaContext,
    pub table: Option<mlua::Table>,
    pub save_data: Vec<u8>,
    pub data_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    pub lua_string: String,
}

impl DevTest {
    /// Creates a new DevTest
    pub fn new() -> Self {
        DevTest {
            lua: LuaContext::new(),
            table: None,
            save_data: vec![],
            data_channel: channel(),
            lua_string: String::new(),
        }
    }
}