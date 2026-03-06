#![windows_subsystem = "windows"]

use std::u8;

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
    numbers: Vec<u16>,
    count: u16,
    state: SortingState,
    algorithm: Algorithm,
}

impl Default for AlgorithmVisualizer {
    fn default() -> Self {
        let mut numbers = (1..=100).collect::<Vec<u16>>();
        fastrand::shuffle(&mut numbers);

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

            if ui
                .add_enabled(is_stopped, egui::Button::image(randomize_icon))
                .on_hover_text("Randomize")
                .clicked()
            {
                fastrand::shuffle(&mut self.numbers);
            }

            ui.add(egui::Button::image(resume_pause_icon))
                .on_hover_text(resume_pause_tooltip);

            ui.add_enabled(!is_stopped, egui::Button::image(stop_icon))
                .on_hover_text("Stop");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(is_stopped, egui::Slider::new(&mut self.count, 10..=1000))
                    .on_hover_text("Number of elements")
                    .dragged()
                {
                    self.numbers = (1..=self.count).collect();
                }
            });
        });
    }

    fn sorting_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let area = response.rect;
            let bar_width = area.width() / self.count as f32;
            let bar_height_per_size = area.height() / self.count as f32;
            self.numbers.iter().enumerate().for_each(|(index, number)| {
                let left_x = area.min.x + bar_width * index as f32;
                let right_x = left_x + bar_width;
                let bottom_y = area.max.y;
                let top_y = area.max.y - bar_height_per_size * *number as f32;

                let bar = egui::Rect::from_two_pos(
                    egui::pos2(left_x, bottom_y),
                    egui::pos2(right_x, top_y),
                );
                painter.rect_filled(bar, 0., egui::Color32::from_gray(u8::MAX));
            });
        });
    }
}

impl eframe::App for AlgorithmVisualizer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("options")
            .show_separator_line(false)
            .show(ctx, |ui| self.options_panel(ui));

        egui::CentralPanel::default().show(ctx, |ui| self.sorting_panel(ui));
    }
}
