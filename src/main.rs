
use bal_save::MyApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "browse",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}