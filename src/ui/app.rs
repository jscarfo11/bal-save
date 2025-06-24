use crate::enums::{PopupType, SaveType, TabState};
use crate::lua::LuaContext;
use crate::saves::{Meta, DevTest};
use crate::ui::Popup;
use crate::ui::drawings;
use egui::Context;

use std::future::Future;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct MyApp {
    save_channel: (Sender<SaveType>, Receiver<SaveType>),
    popup_channel: (Sender<Popup>, Receiver<Popup>),
    save: Option<SaveType>,
    popup: Option<Popup>,
    tab: TabState,
    dark_mode: bool,
    pub dev: DevTest,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            save_channel: channel(),
            popup_channel: channel(),
            save: None,
            popup: None,
            tab: TabState::None,
            dark_mode: true,
            dev: DevTest::new(),
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        MyApp::default()
    }

    fn make_meta(&mut self, ui: &egui::Ui) {
        let meta_sender = self.save_channel.0.clone();
        let popup_sender = self.popup_channel.0.clone();
        let task = rfd::AsyncFileDialog::new().pick_file();
        // Context is wrapped in an Arc so it's cheap to clone as per:
        // > Context is cheap to clone, and any clones refers to the same mutable data (Context uses refcounting internally).
        // Taken from https://docs.rs/egui/0.24.1/egui/struct.Context.html
        let ctx = ui.ctx().clone();
        execute(async move {
            let file = task.await;
            if let Some(file) = file {
                let text = file.read().await;
                let lua_context = LuaContext::new();
                

                // let _ = lua_context
                //     .make_meta_defaults(text.clone());

                let meta = Meta::from_lua_table(lua_context, text.clone());
                if meta.is_err() {
                    let err = meta.err().unwrap();
                    popup_sender
                        .send(Popup::new(PopupType::ErrorLoad, err.to_string()))
                        .unwrap();
                    return;
                }

                let _ = meta_sender.send(SaveType::Meta(meta.unwrap()));
                ctx.request_repaint();
            }
        });
    }

    fn make_dev(&mut self) {

        let dev_sender = self.dev.data_channel.0.clone();
        let task = rfd::AsyncFileDialog::new().pick_file();

        execute(async move {
            let file = task.await;
            if let Some(file) = file {
                let text = file.read().await;
                dev_sender.send(text).unwrap();
            }
    });

    }

    fn handle_popops(&mut self, ctx: &Context) {
        if self.popup.is_none() {
            return;
        }
        let popup = self.popup.as_mut().unwrap();

        let popup_text = popup.get_message().clone();

        match popup.get_type() {
            PopupType::ErrorSave => {
                let modal = egui::Modal::new(egui::Id::new(
                    "Error Saving File",
                ))
                .show(ctx, |ui| {
                    ui.label("Error Saving File");
                    ui.horizontal(|ui| {
                        ui.label(popup_text);

                        if ui.button("Close").clicked() {
                            self.popup = None;
                        }
                    });
                });
                if modal.should_close() {
                    self.popup = None;
                }
            }
            PopupType::ErrorLoad => {
                let mut error_text = popup_text.clone();
                match popup_text.as_str() {
                    "corrupt deflate stream" => {
                        error_text = "Make sure the file you selected is a valid Balatro save file".to_string();
                    }
                    _ if popup_text
                        .contains("stream did not contain valid UTF-8") =>
                    {
                        error_text = "Make sure the file you selected is a valid Balatro save file".to_string();
                    }

                    _ if popup_text.contains("runtime error: Subtable") => {
                        error_text =
                            "Make sure the file you selected is a Meta file"
                                .to_string();
                    }

                    _ => {} // Leave error text as is
                }
                let modal = egui::Modal::new(egui::Id::new(
                    "Error Loading File",
                ))
                .show(ctx, |ui| {
                    ui.label("Error Loading File");
                    ui.horizontal(|ui| {
                        ui.label(error_text);

                        if ui.button("Close").clicked() {
                            self.popup = None;
                        }
                    });
                });
                if modal.should_close() {
                    self.popup = None;
                }
            }
            PopupType::ConfirmMetaDefault => {
                let modal = egui::Modal::new(egui::Id::new(
                    "Confirm Default Meta",
                ))
                .show(ctx, |ui| {
                    ui.label("Are you sure you want to load the default meta? This will overwrite your current meta.");
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.save = Some(SaveType::Meta(Meta::from_defaults()));
                            self.tab = TabState::Editor;
                            self.popup = None;
                        }
                        if ui.button("No").clicked() {
                            self.popup = None;
                        }
                    });
                });
                if modal.should_close() {
                    self.popup = None;
                }
            }
            PopupType::ConfirmMetaFile => {
                let modal = egui::Modal::new(egui::Id::new(
                    "Confirm Meta Overwrite",)).show(ctx, |ui| {
                    ui.label("Are you sure you want to load another meta file? This will overwrite your current meta.");
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.popup = None;
                            self.make_meta(ui);
                        }
                        if ui.button("No").clicked() {
                            self.popup = None;
                        }
                    })
            });
                if modal.should_close() {
                    self.popup = None;
                }
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // assign sample text once it comes in

        if let Ok(save) = self.save_channel.1.try_recv() {
            self.save = Some(save);
            self.tab = TabState::Editor;
        }
        if let Ok(popup) = self.popup_channel.1.try_recv() {
            self.popup = Some(popup);
        }

        if let Ok(dev) = self.dev.data_channel.1.try_recv() {
            self.dev.save_data = dev;
            self.dev.table = None;
        }

        self.handle_popops(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.tab == TabState::None, "File IO")
                    .clicked()
                {
                    self.tab = TabState::None;
                }
                if ui
                    .selectable_label(self.tab == TabState::Editor, "Editor")
                    .clicked()
                {
                    self.tab = TabState::Editor;
                }
                if ui
                    .selectable_label(
                        self.tab == TabState::Settings,
                        "Settings",
                    )
                    .clicked()
                {
                    self.tab = TabState::Settings;
                }
                if ui
                    .selectable_label(self.tab == TabState::Help, "Help")
                    .clicked()
                {
                    self.tab = TabState::Help;
                }
                if ui
                    .selectable_label(self.tab == TabState::Dev, "Dev")
                    .clicked()
                {
                    self.tab = TabState::Dev;
                }

                if ui.selectable_label(self.dark_mode, "🌗").clicked() {
                    self.dark_mode = !self.dark_mode;
                    ctx.set_visuals(if self.dark_mode {
                        egui::Visuals::dark()
                    } else {
                        egui::Visuals::light()
                    });
                }
            });



            match self.tab {
                TabState::None => {
                    if ui.button("📂 Open Meta file").clicked() {
                        if self.save.is_none() {
                            self.make_meta(ui);
                        } else {
                            self.popup = Some(Popup::new(
                                PopupType::ConfirmMetaFile,
                                "".to_string(),
                            ));
                        }
                    }

                    if ui.button("📂 Open Dev File").clicked() {
                        if self.save.is_none() {
                            self.make_dev();
                        } else {
                            panic!()
                        }
                    }

                    if ui.button("💾 Save Editor to File").clicked() {
                        if self.save.is_none() {
                            self.popup = Some(Popup::new(
                                PopupType::ErrorSave,
                                "Error: No Save Loaded".to_string(),
                            ));
                            return;
                        }

                        let save = self.save.as_mut().unwrap();

                        match save {
                            SaveType::Meta(meta) => {
                                let lua_context = LuaContext::new();
                                let x = meta.to_lua_data(&lua_context);
                                match x {
                                    Ok(x) => {
                                        let task = rfd::AsyncFileDialog::new()
                                            .save_file();
                                        execute(async move {
                                            let file = task.await;
                                            if let Some(file) = file {
                                                // let file_content = self.meta.to_lua_data();
                                                _ = file.write(&x).await;
                                            }
                                        });
                                    }
                                    Err(err) => {
                                        self.popup = Some(Popup::new(
                                            PopupType::ErrorSave,
                                            err.to_string(),
                                        ));
                                    }
                                }
                            }

                            SaveType::Profile(profile) => {
                                let lua_context = LuaContext::new();
                                let x = profile.to_lua_data(&lua_context);
                                match x {
                                    Ok(x) => {
                                        let task = rfd::AsyncFileDialog::new()
                                            .save_file();
                                        execute(async move {
                                            let file = task.await;
                                            if let Some(file) = file {
                                                // let file_content = self.meta.to_lua_data();
                                                _ = file.write(&x).await;
                                            }
                                        });
                                    }
                                    Err(err) => {
                                        self.popup = Some(Popup::new(
                                            PopupType::ErrorSave,
                                            err.to_string(),
                                        ));
                                    }
                                }
                            }
                        }





                    }

                    if ui.button("❓ Default Meta").clicked() {
                        if self.save.is_none() {
                            self.save = Some(SaveType::Meta(Meta::from_defaults()));
                            self.tab = TabState::Editor;
                        } else {
                            self.popup = Some(Popup::new(
                                PopupType::ConfirmMetaDefault,
                                "Are you sure you want to load the default meta? This will overwrite your current meta.".to_string(),
                            ));
                        }
                    }
                }

                TabState::Editor => {
                    ui.horizontal(|ui| {
                        if self.save.is_none() {
                            ui.label("No Save Loaded");
                            return;
                        }

                        match self.save.as_mut().unwrap() {
                            SaveType::Meta(meta) => {

                            drawings::draw_meta(meta, ctx, ui);
                            }

                            SaveType::Profile(profile) => {
                                drawings::draw_profile(profile, ctx, ui);
                            }
                        }
                    });
                }

                TabState::Settings => {
                    ui.label("Settings");
                    ui.horizontal(|ui| {
                        ui.label("No Settings yet");
                    });
                }

                TabState::Help => {
                    ui.label("Help");
                    ui.horizontal(|ui| {
                        ui.label("No Help yet");
                    });
                }
                TabState::Dev => {
                    ui.label("Dev");
                    drawings::draw_dev(self, ui);
                }
            }

        });
    }
}

// fn decompress_data(data: Vec<u8>) -> Result<String, std::io::Error> {
//     let mut decoder = flate2::read::DeflateDecoder::new(&data[..]);
//     let mut s = String::new();
//     decoder.read_to_string(&mut s)?;
//     let lua = Lua::new();
//     let val: Value = lua.load(&s).eval().unwrap();

//     lua.globals().set("map_table", &val).unwrap();

//     let alerted_table = access_subtable(val, "alerted").unwrap();
//     lua.globals().set("alerted_table", &alerted_table).unwrap();

//     lua.load("for k,v in pairs(alerted_table) do print(k,v) end").exec().unwrap();
//     // lua.load("for k,v in pairs(map_table) do print(k,v) end").exec().unwrap();

//     Ok(s)
// }

fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}
