use crate::helpers::LuaContext;
use crate::helpers::Meta;
use crate::helpers::TabState;
use egui::Context;
use inflector::Inflector;
use std::future::Future;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct MyApp {
    text_channel: (Sender<String>, Receiver<String>),
    meta_channel: (Sender<Meta>, Receiver<Meta>),
    save_text: String,
    meta: Option<Meta>,
    // test: bool,
    tab: TabState,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_channel: channel(),
            meta_channel: channel(),
            save_text: "".into(),
            meta: None,
            tab: TabState::None,
            // test: false,
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        MyApp::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // assign sample text once it comes in
        if let Ok(text) = self.text_channel.1.try_recv() {
            self.save_text = text;
        }

        if let Ok(meta) = self.meta_channel.1.try_recv() {
            self.meta = Some(meta);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.tab == TabState::None, "Open File")
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
                    .selectable_label(self.tab == TabState::Settings, "Settings")
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
            });

            match self.tab {
                TabState::None => {
                    if ui.button("📂 Open text file").clicked() {
                        let sender = self.text_channel.0.clone();
                        let meta_sender = self.meta_channel.0.clone();
                        let task = rfd::AsyncFileDialog::new().pick_file();
                        // Context is wrapped in an Arc so it's cheap to clone as per:
                        // > Context is cheap to clone, and any clones refers to the same mutable data (Context uses refcounting internally).
                        // Taken from https://docs.rs/egui/0.24.1/egui/struct.Context.html
                        let ctx = ui.ctx().clone();
                        self.tab = TabState::Editor;
                        execute(async move {
                            let file = task.await;
                            if let Some(file) = file {
                                let text = file.read().await;
                                let lua_context = LuaContext::new();
                                let _ = sender.send(decompress_data(text.clone()).unwrap());
                                ctx.request_repaint();

                                let meta = Meta::from_lua_table(lua_context, text);

                                // let alerted = lua_context.access_subtable(&entire_file_table, "alerted").unwrap();
                                // let discovered = lua_context.access_subtable(&entire_file_table, "discovered").unwrap();
                                // let unlocked = lua_context.access_subtable(&entire_file_table, "unlocked").unwrap();

                                // for pair in alerted.pairs::<String, bool>() {
                                //     match pair {

                                //         Ok((key, value)) => {
                                //             let start = "e_";
                                //             let cry = start.to_string() + "cry_";
                                //             let mp = start.to_string() + "mp_";
                                //             let mtg = start.to_string() + "mtg_";
                                //             let alerted_val = value;
                                //             let discovered_val = discovered.get(key.clone()).unwrap_or(false);
                                //             let unlocked_val = unlocked.get(key.clone()).unwrap_or(false);

                                //             if key.starts_with(start) && !key.starts_with(&cry) && !key.starts_with(&mp) && !key.starts_with(&mtg) {
                                //                 println!("(\"{}\", {}, {}, {}),", key, alerted_val, discovered_val, unlocked_val);
                                //             }

                                //             // e_

                                //             // j_
                                //             // v_
                                //             // b_
                                //             // c_

                                //         }

                                //         Err(err) => eprintln!("Error iterating over alerted pairs: {}", err),
                                //     }
                                // }

                                let _ = meta_sender.send(meta);
                            }
                        });
                    }

                    if ui.button("💾 Save text to file").clicked() {
                        let task = rfd::AsyncFileDialog::new().save_file();
                        let contents = self.save_text.clone();
                        execute(async move {
                            let file = task.await;
                            if let Some(file) = file {
                                _ = file.write(contents.as_bytes()).await;
                            }
                        });
                    }
                }

                TabState::Editor => {
                    ui.label("Editor");
                    ui.horizontal(|ui| {
                        ui.label("Jokers");

                        ui.separator();

                        ui.label("Vouchers");
                    });

                    match &mut self.meta {
                        Some(meta) => {
                            if ui.button("Unlock All Jokers").clicked() {
                                meta.unlock_all_jokers();
                            }

                            if ui.button("Unlock All Vouchers").clicked() {
                                meta.unlock_all_vouchers();
                            }

                            if ui.button("Unlock All Cards").clicked() {
                                meta.unlock_all_cards();
                            }

                            if ui.button("Unlock All Enchantments").clicked() {
                                meta.unlock_all_enchancements();
                            }

                            if ui.button("Unlock All Decks").clicked() {
                                meta.unlock_all_decks();
                            }
                            let window_size = ctx.screen_rect().size();
                            egui::containers::ScrollArea::vertical()
                                .max_height(window_size.y * 0.2)
                                .max_width(2000.0)
                                .id_salt(1)
                                .show(ui, |ui| {
                                    let mut joker_names = meta.get_joker_names();
                                    joker_names.sort();
                                    for joker_name in joker_names.iter() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "{}",
                                                &joker_name[2..].to_title_case()
                                            ));
                                            let joker = meta.get_joker(&joker_name);

                                            let joker_val = joker.unwrap();

                                            ui.checkbox(&mut joker_val.alerted, "Alerted");
                                            ui.checkbox(&mut joker_val.discovered, "Discovered");
                                            ui.checkbox(&mut joker_val.unlocked, "Unlocked");
                                        });
                                    }
                                });
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);
                            egui::containers::ScrollArea::vertical()
                                .max_height(window_size.y * 0.2)
                                .max_width(2000.0)
                                .id_salt(2)
                                .show(ui, |ui| {
                                    let mut voucher_names = meta.get_voucher_names();
                                    voucher_names.sort();
                                    for voucher_name in voucher_names.iter() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "{}",
                                                &voucher_name[2..].to_title_case()
                                            ));
                                            let voucher = meta.get_voucher(&voucher_name);

                                            let voucher_val = voucher.unwrap();

                                            ui.checkbox(&mut voucher_val.alerted, "Alerted");
                                            ui.checkbox(&mut voucher_val.discovered, "Discovered");
                                            ui.checkbox(&mut voucher_val.unlocked, "Unlocked");
                                        });
                                    }
                                });
                        }
                        None => {
                            ui.label("No Meta yet");
                        }
                    }
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

fn decompress_data(data: Vec<u8>) -> Result<String, std::io::Error> {
    let mut decoder = flate2::read::DeflateDecoder::new(&data[..]);
    let mut s = String::new();
    decoder.read_to_string(&mut s)?;
    // let lua = Lua::new();
    // let val: Value = lua.load(&s).eval().unwrap();

    // lua.globals().set("map_table", &val).unwrap();

    // let alerted_table = access_subtable(val, "alerted").unwrap();
    // lua.globals().set("alerted_table", &alerted_table).unwrap();

    // lua.load("for k,v in pairs(alerted_table) do print(k,v) end").exec().unwrap();
    // // lua.load("for k,v in pairs(map_table) do print(k,v) end").exec().unwrap();

    Ok(s)
}

fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
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
