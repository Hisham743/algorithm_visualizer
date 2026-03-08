#![feature(gen_blocks)]

mod algorithms;

use eframe::egui;
use std::time::Duration;

use algorithms::{Algorithm, Snapshot};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortingState {
    Idle,
    Running,
    Paused,
}

pub struct AlgorithmVisualizer {
    count: u16,
    state: SortingState,
    algorithm: Algorithm,
    snapshot: Snapshot<u16>,
    steps: Box<dyn Iterator<Item = Snapshot<u16>>>,
}

impl Default for AlgorithmVisualizer {
    fn default() -> Self {
        let mut numbers = (1..=100).collect::<Vec<u16>>();
        fastrand::shuffle(&mut numbers);

        let snapshot = Snapshot {
            numbers: numbers.clone(),
            active_element: None,
        };

        AlgorithmVisualizer {
            count: 100,
            state: SortingState::Idle,
            algorithm: Algorithm::Bubble,
            snapshot,
            steps: Box::new(Algorithm::Bubble.steps()(numbers)),
        }
    }
}

impl AlgorithmVisualizer {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::default()
    }

    fn options_panel(&mut self, ui: &mut egui::Ui) {
        let randomize_icon = egui::include_image!("./images/randomize.png");
        let resume_icon = egui::include_image!("./images/resume.png");
        let pause_icon = egui::include_image!("./images/pause.png");
        let stop_icon = egui::include_image!("./images/stop.png");

        let (resume_pause_icon, resume_pause_tooltip, next_state) =
            if self.state == SortingState::Running {
                (pause_icon, "Pause", SortingState::Paused)
            } else {
                (resume_icon, "Sort", SortingState::Running)
            };

        let is_stopped = self.state == SortingState::Idle;

        ui.horizontal(|ui| {
            ui.add_enabled_ui(is_stopped, |ui| {
                let before = self.algorithm;

                egui::ComboBox::from_id_salt("Algorithm")
                    .selected_text(self.algorithm.to_string())
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

                if self.algorithm != before {
                    self.steps = self.algorithm.steps()(self.snapshot.numbers.clone())
                }
            });

            if ui
                .add_enabled(is_stopped, egui::Button::image(randomize_icon))
                .on_hover_text("Randomize")
                .clicked()
            {
                fastrand::shuffle(&mut self.snapshot.numbers);
                self.steps = self.algorithm.steps()(self.snapshot.numbers.clone())
            }

            if ui
                .add(egui::Button::image(resume_pause_icon))
                .on_hover_text(resume_pause_tooltip)
                .clicked()
            {
                self.state = next_state;
            }

            if ui
                .add_enabled(!is_stopped, egui::Button::image(stop_icon))
                .on_hover_text("Stop")
                .clicked()
            {
                self.state = SortingState::Idle;
                self.steps = self.algorithm.steps()(self.snapshot.numbers.clone());
                self.snapshot.active_element = None;
            };

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(is_stopped, egui::Slider::new(&mut self.count, 10..=1000))
                    .on_hover_text("Number of elements")
                    .changed()
                {
                    self.snapshot.numbers = (1..=self.count).collect();
                    self.steps = self.algorithm.steps()(self.snapshot.numbers.clone())
                }
            });
        });
    }

    fn sorting_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            if self.state == SortingState::Running {
                let step = self.steps.next();

                match step {
                    Some(snapshot) => self.snapshot = snapshot,
                    None => {
                        self.state = SortingState::Idle;
                        self.steps = self.algorithm.steps()(self.snapshot.numbers.clone())
                    }
                }
            }

            let area = response.rect;
            let bar_width = area.width() / self.count as f32;
            let bar_height_per_size = area.height() / self.count as f32;
            self.snapshot
                .numbers
                .iter()
                .enumerate()
                .for_each(|(index, number)| {
                    let left_x = area.min.x + bar_width * index as f32;
                    let right_x = left_x + bar_width;
                    let bottom_y = area.max.y;
                    let top_y = area.max.y - bar_height_per_size * *number as f32;

                    let bar = egui::Rect::from_two_pos(
                        egui::pos2(left_x, bottom_y),
                        egui::pos2(right_x, top_y),
                    );

                    let colour = if let Some(active_index) = self.snapshot.active_element
                        && active_index == index
                    {
                        egui::Color32::from_rgb(u8::MAX, 0, 0)
                    } else {
                        egui::Color32::from_gray(u8::MAX)
                    };

                    painter.rect_filled(bar, 0., colour);
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

        ctx.request_repaint_after(Duration::from_millis(1000 / 60_u64));
    }
}
