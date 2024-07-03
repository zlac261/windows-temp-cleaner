mod app;
mod file_operations;

use std::fs;
use std::path::Path;

fn is_admin() -> bool {
    let test_path = Path::new("C:\\Windows\\Temp\\test_admin_access.txt");
    match fs::write(test_path, b"test") {
        Ok(_) => {
            let _ = fs::remove_file(test_path);
            true
        }
        Err(_) => false,
    }
}

fn main() -> Result<(), eframe::Error> {
    if !is_admin() {
        println!("This application requires admin privileges");
        println!("Please run again as admin");
        std::process::exit(1);
    }


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