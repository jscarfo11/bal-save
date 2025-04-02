use egui::Context;
use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::io::Read;
use crate::helpers::LuaContext;
use crate::helpers::Meta;

pub struct MyApp {
    text_channel: (Sender<String>, Receiver<String>),
    save_text: String,
    meta: Option<Meta>,
    test: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_channel: channel(),
            save_text: "".into(),
            meta: None,
            test: false,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                
                ui.text_edit_multiline(&mut self.save_text);
                ui.checkbox(&mut self.test, "I am a checkbox")
                
                // ui.checkbox(checked, text)
            });
            // a simple button opening the dialog
            if ui.button("📂 Open text file").clicked() {
                let sender = self.text_channel.0.clone();
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
                        let _ = sender.send(decompress_data(text.clone()).unwrap());
                        ctx.request_repaint();


                        let entire_file_table = lua_context.data_as_table(text, "map_table").unwrap();

                        let alerted = lua_context.access_subtable(&entire_file_table, "alerted").unwrap();
                        let discovered = lua_context.access_subtable(&entire_file_table, "discovered").unwrap();
                        let unlocked = lua_context.access_subtable(&entire_file_table, "unlocked").unwrap();

                        for pair in alerted.pairs::<String, bool>() {
                            match pair {
                                // Ok((key, value)) => {
                                //     if key.starts_with("b_") && !key.starts_with("b_cry_") && !key.starts_with("b_mp_") && !key.starts_with("b_mtg_") {
                                //         println!("{}: {}", key, value);
                                //     }
                                Ok((key, value)) => {
                                    let start = "e_";
                                    let cry = start.to_string() + "cry_";
                                    let mp = start.to_string() + "mp_";
                                    let mtg = start.to_string() + "mtg_";
                                    let alerted_val = value;
                                    let discovered_val = discovered.get(key.clone()).unwrap_or(false);
                                    let unlocked_val = unlocked.get(key.clone()).unwrap_or(false);

                                    if key.starts_with(start) && !key.starts_with(&cry) && !key.starts_with(&mp) && !key.starts_with(&mtg) {
                                        println!("(\"{}\", {}, {}, {}),", key, alerted_val, discovered_val, unlocked_val);
                                    }

                                    
                                    
                                    // e_


                                    // j_
                                    // v_
                                    // b_
                                    // c_
                                
                                }

                                    
                                Err(err) => eprintln!("Error iterating over alerted pairs: {}", err),
                            }
                        }



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