use bal_save::MyApp;

fn main() -> eframe::Result<()> {
    

    let mut native_options = eframe::NativeOptions::default();
    let viewport = egui::ViewportBuilder::default().with_title("bal_save").with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("assets/temp_logo.png")[..])
    .unwrap());

    native_options.viewport = viewport;
    eframe::run_native(
        "bal_save",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
