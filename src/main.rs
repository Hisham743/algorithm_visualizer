#![windows_subsystem = "windows"]

use std::iter;

use eframe::egui;

fn main() -> eframe::Result {
    let app = AlgorithmVisualizer::default();
    eframe::run_native(
        "Algorithm Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(app))),
    )
}

struct AlgorithmVisualizer {
    numbers: Vec<u8>,
    is_sorting: bool,
}

impl Default for AlgorithmVisualizer {
    fn default() -> Self {
        let numbers = iter::repeat_with(|| fastrand::u8(..)).take(100).collect();

        AlgorithmVisualizer {
            numbers,
            is_sorting: false,
        }
    }
}

impl eframe::App for AlgorithmVisualizer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
