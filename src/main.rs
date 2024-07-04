#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod file_operations;
mod loading_bar;
mod log_display;
mod file_type;

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

    let app_creator = move |cc: &CreationContext| -> Result<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>> {
        if is_admin() {
            Ok(Box::new(app::TempFileCleanerApp::new(cc)))
        } else {
            Ok(Box::new(AdminPrompt))
        }
    };

    eframe::run_native(
        if is_admin() { "Windows Temp Cleaner" } else { "Administrator Required" },
        options,
        Box::new(app_creator),
    )
}