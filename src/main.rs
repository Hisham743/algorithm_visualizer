#![windows_subsystem = "windows"]

use std::iter;

use algorithm_visualizer::Algorithm;
use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Algorithm Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(AlgorithmVisualizer::new(cc)))),
    )
}

enum SortingState {
    Running,
    Paused,
    Stopped,
}

struct AlgorithmVisualizer {
    numbers: Vec<u8>,
    count: usize,
    state: SortingState,
    algorithm: Algorithm,
}

impl Default for AlgorithmVisualizer {
    fn default() -> Self {
        let numbers = iter::repeat_with(|| fastrand::u8(..)).take(100).collect();

        AlgorithmVisualizer {
            numbers,
            count: 100,
            state: SortingState::Stopped,
            algorithm: Algorithm::Bubble,
        }
    }
}

impl AlgorithmVisualizer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::default()
    }

    fn options_panel(&mut self, ui: &mut egui::Ui) {
        let randomize_icon = egui::include_image!("./images/randomize.png");
        let resume_icon = egui::include_image!("./images/resume.png");
        let pause_icon = egui::include_image!("./images/pause.png");
        let stop_icon = egui::include_image!("./images/stop.png");

        let (resume_pause_icon, resume_pause_tooltip) =
            if matches!(self.state, SortingState::Running) {
                (pause_icon, "Pause")
            } else {
                (resume_icon, "Sort")
            };

        let is_stopped = matches!(self.state, SortingState::Stopped);

        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("Algorithm")
                .selected_text(&self.algorithm.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.algorithm,
                        Algorithm::Bubble,
                        Algorithm::Bubble.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.algorithm,
                        Algorithm::Selection,
                        Algorithm::Selection.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.algorithm,
                        Algorithm::Insertion,
                        Algorithm::Insertion.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.algorithm,
                        Algorithm::Merge,
                        Algorithm::Merge.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.algorithm,
                        Algorithm::Quick,
                        Algorithm::Quick.to_string(),
                    );
                })
                .response
                .on_hover_text("Algorithm");

            ui.add_enabled(is_stopped, egui::Button::image(randomize_icon))
                .on_hover_text("Randomize");

            ui.add(egui::Button::image(resume_pause_icon))
                .on_hover_text(resume_pause_tooltip);

            ui.add_enabled(!is_stopped, egui::Button::image(stop_icon))
                .on_hover_text("Stop");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_enabled(is_stopped, egui::Slider::new(&mut self.count, 10..=1000))
                    .on_hover_text("Number of elements");
            });
        });
    }
}

impl eframe::App for AlgorithmVisualizer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("options")
            .show_separator_line(false)
            .show(ctx, |ui| self.options_panel(ui));

        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
