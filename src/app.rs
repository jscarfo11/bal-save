use crate::enums::{PopupType, TabState};
use crate::lua::LuaContext;
use crate::saves::Meta;
use crate::ui::Popup;

use egui::Context;
use egui::Label;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use inflector::Inflector;
use std::future::Future;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct MyApp {
    meta_channel: (Sender<Meta>, Receiver<Meta>),
    popup_channel: (Sender<Popup>, Receiver<Popup>),
    meta: Option<Meta>,
    popup: Option<Popup>,
    tab: TabState,
    matcher: SkimMatcherV2,
    dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            meta_channel: channel(),
            popup_channel: channel(),
            meta: None,
            popup: None,
            tab: TabState::None,
            matcher: SkimMatcherV2::default(),
            dark_mode: true,
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        MyApp::default()
    }

    fn make_meta(&mut self, ui: &egui::Ui) {
        let meta_sender = self.meta_channel.0.clone();
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
                ctx.request_repaint();

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

                let _ = meta_sender.send(meta.unwrap());
            }
        });
    }
    fn draw_meta(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        if self.meta.is_none() {
            ui.label("No Save Loaded");
            return;
        }
        let meta = self.meta.as_mut().unwrap();

        let window_size = ctx.screen_rect().size();
        let num_columns = 2;
        let scroll_height = window_size.y * 0.4;
        let search_width = window_size.x / num_columns as f32 * 0.65;

        ui.columns(num_columns, |columns| {
            columns[0].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add(Label::new(
                    egui::RichText::new("Jokers").color(egui::Color32::GREEN),
                ));
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut meta.filters.joker)
                            .desired_width(search_width)
                            .hint_text("Filter Jokers"),
                    );
                    if ui.button("Unlock All").clicked() {
                        meta.unlock_all_type("j_");
                    }
                });

                egui::containers::ScrollArea::both()
                    .auto_shrink(false)
                    // .max_height(scroll_height)
                    .min_scrolled_height(scroll_height)
                    .id_salt("Joker Table")
                    .show(ui, |ui| {
                        let mut joker_names = meta.get_joker_names();

                        if meta.filters.joker != "" {
                            joker_names.retain(|name| {
                                let score = self.matcher.fuzzy_match(
                                    &name[2..].to_lowercase(),
                                    &meta.filters.joker.to_lowercase(),
                                );
                                if score.is_some_and(|x| x > 1) {
                                    return true;
                                }
                                return false;
                            });
                        }
                        for joker_name in joker_names.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", &joker_name[2..].to_title_case()));
                                let joker = meta.get_item(&joker_name);

                                let joker_val = joker.unwrap();

                                if joker_val.can_be_alerted() {
                                    ui.checkbox(&mut joker_val.alerted, "Alerted");
                                }

                                if joker_val.can_be_discovered() {
                                    ui.checkbox(&mut joker_val.discovered, "Discovered");
                                }

                                if joker_val.can_be_unlocked() {
                                    ui.checkbox(&mut joker_val.unlocked, "Unlocked");
                                }
                            });
                        }
                    });

                ui.separator();

                ui.add(Label::new(
                    egui::RichText::new("Cards").color(egui::Color32::LIGHT_BLUE),
                ));
                ui.horizontal(|ui| {
                    // ui.label("Search");

                    ui.add(
                        egui::TextEdit::singleline(&mut meta.filters.card)
                            .desired_width(search_width)
                            .hint_text("Filter Cards"),
                    );
                    if ui.button("Unlock All").clicked() {
                        meta.unlock_all_type("c_");
                    }
                });
                egui::containers::ScrollArea::both()
                    .auto_shrink(false)
                    .max_height(scroll_height)
                    .min_scrolled_height(scroll_height)
                    .id_salt("Card Table")
                    .show(ui, |ui| {
                        let mut card_names = meta.get_card_names();

                        if meta.filters.card != "" {
                            card_names.retain(|name| {
                                let score = self.matcher.fuzzy_match(
                                    &name[2..].to_lowercase(),
                                    &meta.filters.card.to_lowercase(),
                                );
                                if score.is_some_and(|x| x > 1) {
                                    return true;
                                }
                                return false;
                            });
                        }

                        for card_name in card_names.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", &card_name[2..].to_title_case()));
                                let card = meta.get_item(&card_name);

                                let card_val = card.unwrap();

                                if card_val.can_be_alerted() {
                                    ui.checkbox(&mut card_val.alerted, "Alerted");
                                }
                                if card_val.can_be_discovered() {
                                    ui.checkbox(&mut card_val.discovered, "Discovered");
                                }
                                if card_val.can_be_unlocked() {
                                    ui.checkbox(&mut card_val.unlocked, "Unlocked");
                                }
                            });
                        }
                    });
                ui.separator();
            });

            columns[1].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add(Label::new(
                    egui::RichText::new("Vouchers").color(egui::Color32::PURPLE),
                ));
                ui.horizontal(|ui| {
                    // ui.label("Search");

                    ui.add(
                        egui::TextEdit::singleline(&mut meta.filters.voucher)
                            .desired_width(search_width)
                            .hint_text("Filter Vouchers"),
                    );
                    if ui.button("Unlock All").clicked() {
                        meta.unlock_all_type("v_");
                    }
                });
                egui::containers::ScrollArea::both()
                    .auto_shrink(false)
                    .max_height(scroll_height)
                    .min_scrolled_height(scroll_height)
                    .id_salt("Voucher Table")
                    .show(ui, |ui| {
                        let mut voucher_names = meta.get_voucher_names();

                        if meta.filters.voucher != "" {
                            voucher_names.retain(|name| {
                                let score = self.matcher.fuzzy_match(
                                    &name[2..].to_lowercase(),
                                    &meta.filters.voucher.to_lowercase(),
                                );
                                if score.is_some_and(|x| x > 1) {
                                    return true;
                                }
                                false
                            });
                        }
                        for voucher_name in voucher_names.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", &voucher_name[2..].to_title_case()));
                                let voucher = meta.get_item(&voucher_name);

                                let voucher_val = voucher.unwrap();

                                if voucher_val.can_be_alerted() {
                                    ui.checkbox(&mut voucher_val.alerted, "Alerted");
                                }
                                if voucher_val.can_be_discovered() {
                                    ui.checkbox(&mut voucher_val.discovered, "Discovered");
                                }
                                if voucher_val.can_be_unlocked() {
                                    ui.checkbox(&mut voucher_val.unlocked, "Unlocked");
                                }
                            });
                        }
                    });
                ui.separator();
                ui.add(Label::new(
                    egui::RichText::new("Misc").color(egui::Color32::LIGHT_RED),
                ));
                ui.horizontal(|ui| {
                    // ui.label("Search");

                    ui.add(
                        egui::TextEdit::singleline(&mut meta.filters.misc)
                            .desired_width(search_width)
                            .hint_text("Filter Decks, Blinds, Tags, Edtions, and Booster Packs"),
                    );
                    if ui.button("Unlock All").clicked() {
                        meta.unlock_all_type("b_");
                        meta.unlock_all_type("e_");
                        meta.unlock_all_type("tag_");
                        meta.unlock_all_type("p_");
                        meta.unlock_all_type("bl_");
                    }

                });

                egui::containers::ScrollArea::both()
                    .auto_shrink(false)
                    .max_height(scroll_height)
                    .min_scrolled_height(scroll_height)
                    .id_salt("Deck Table")
                    .show(ui, |ui| {
                        let mut misc_names = meta.get_misc_names();

                        if meta.filters.misc != "" {
                            misc_names.retain(|name| {
                                let score = self.matcher.fuzzy_match(
                                    &name[2..].to_lowercase(),
                                    &meta.filters.misc.to_lowercase(),
                                );
                                if score.is_some_and(|x| x > 1) {
                                    return true;
                                }
                                return false;
                            });
                        }
                        for deck_name in misc_names.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", &deck_name[2..].to_title_case()));
                                let misc = meta.get_item(&deck_name);

                                let misc_val = misc.unwrap();
                                if misc_val.can_be_alerted() {
                                    ui.checkbox(&mut misc_val.alerted, "Alerted");
                                }
                                if misc_val.can_be_discovered() {
                                    ui.checkbox(&mut misc_val.discovered, "Discovered");
                                }
                                if misc_val.can_be_unlocked() {
                                    ui.checkbox(&mut misc_val.unlocked, "Unlocked");
                                }
                            });
                        }
                    });

                ui.separator();
            });
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
                            self.meta = Some(Meta::from_defaults());
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

        if let Ok(meta) = self.meta_channel.1.try_recv() {
            self.meta = Some(meta);
            self.tab = TabState::Editor;
        }
        if let Ok(popup) = self.popup_channel.1.try_recv() {
            self.popup = Some(popup);
        }

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

                if ui.selectable_label(self.dark_mode, "🌗").clicked() {
                    self.dark_mode = !self.dark_mode;
                    ctx.set_visuals(if self.dark_mode {
                        egui::Visuals::dark()
                    } else {
                        egui::Visuals::light()
                    });
                }
            });

            self.handle_popops(ctx);

            match self.tab {
                TabState::None => {
                    if ui.button("📂 Open Meta file").clicked() {
                        if self.meta.is_none() {
                            self.make_meta(ui);
                        } else {
                            self.popup = Some(Popup::new(
                                PopupType::ConfirmMetaFile,
                                "".to_string(),
                            ));
                        }
                    }

                    if ui.button("💾 Save Meta to File").clicked() {
                        match &self.meta {
                            Some(meta) => {

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
                            None => {
                                self.popup = Some(Popup::new(
                                    PopupType::ErrorSave,
                                    "Error: No Save Loaded".to_string(),
                                ));
                            }
                        }
                    }

                    if ui.button("❓ Default Meta").clicked() {
                        if self.meta.is_none() {
                            self.meta = Some(Meta::from_defaults());
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
                        self.draw_meta(ctx, ui);
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
            }

            // a simple button opening the dialog
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
