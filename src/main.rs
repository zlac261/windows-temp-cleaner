mod app;
mod file_operations;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Windows Temp Cleaner",
        options,
        Box::new(move |cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
            let app = app::TempFileCleanerApp::new(cc);
            Ok(Box::new(app))
        }),
    )
}