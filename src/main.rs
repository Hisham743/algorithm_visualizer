#![windows_subsystem = "windows"]

use algorithm_visualizer::AlgorithmVisualizer;

fn main() -> eframe::Result {
    eframe::run_native(
        "Algorithm Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(AlgorithmVisualizer::new(cc)))),
    )
}
