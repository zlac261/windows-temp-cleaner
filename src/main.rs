mod app;
mod file_operations;

use std::error::Error;
use std::fs;
use std::path::Path;
use eframe::{self, egui, NativeOptions, App, CreationContext};

struct AdminPrompt;

impl eframe::App for AdminPrompt {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Administrator Required");
            ui.label("Run as administrator.");
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }
}

fn is_admin() -> bool {
    let test_path = Path::new("C:\\Windows\\Temp\\test_admin_access.txt");
    fs::write(test_path, b"test").and_then(|_| fs::remove_file(test_path)).is_ok()
}

fn main() -> eframe::Result<()> {
    let options = if !is_admin() {
        NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 100.0]),
            ..Default::default()
        }
    } else {
        NativeOptions::default()
    };

    eframe::run_native(
        if is_admin() { "Windows Temp Cleaner" } else { "Administrator Required" },
        options,
        Box::new(|cc: &CreationContext| -> Result<Box<dyn App>, Box<dyn Error + Send + Sync>> {
            if is_admin() {
                Ok(Box::new(app::TempFileCleanerApp::new(cc)))
            } else {
                Ok(Box::new(AdminPrompt))
            }
        }),
    )
}