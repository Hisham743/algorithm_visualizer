mod algorithms;

use algorithms::{Algorithm, Operation};
use eframe::{
    App, CreationContext,
    egui::{
        self, Align, Button, CentralPanel, ComboBox, Context, DragValue, Layout, Rect, Slider, Ui,
    },
};
use std::{time::Duration, vec::IntoIter};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_sys::Performance;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortingState {
    Idle,
    Running,
    Paused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HiglightedElement {
    index: usize,
    color: egui::Color32,
}

pub struct AlgorithmVisualizer {
    count: u16,
    state: SortingState,
    algorithm: Algorithm,
    speed: u8,
    tick: Duration,
    numbers: Vec<u16>,
    active_elements: (Option<HiglightedElement>, Option<HiglightedElement>),
    operations: IntoIter<Operation>,

    #[cfg(not(target_arch = "wasm32"))]
    last_operation_instant: Instant,

    #[cfg(target_arch = "wasm32")]
    performance: Performance,
    #[cfg(target_arch = "wasm32")]
    last_operation_instant: f64,
}

impl Default for AlgorithmVisualizer {
    fn default() -> Self {
        let mut numbers = (1..=100).collect::<Vec<u16>>();
        fastrand::shuffle(&mut numbers);

        let algorithm = Algorithm::Bubble;
        let operations = algorithm.operations()(numbers.clone());
        let speed = 10;

        #[cfg(target_arch = "wasm32")]
        let performance = web_sys::window()
            .expect("no window")
            .performance()
            .expect("no performance");
        #[cfg(target_arch = "wasm32")]
        let instant = performance.now();

        AlgorithmVisualizer {
            count: 100,
            state: SortingState::Idle,
            algorithm,
            speed,
            tick: Duration::from_millis(1000 / speed as u64),
            numbers,
            active_elements: (None, None),
            operations,

            #[cfg(not(target_arch = "wasm32"))]
            last_operation_instant: Instant::now(),

            #[cfg(target_arch = "wasm32")]
            performance,
            #[cfg(target_arch = "wasm32")]
            last_operation_instant: instant,
        }
    }
}

impl AlgorithmVisualizer {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
        Self::default()
    }

    fn reset_operations(&mut self) {
        self.operations = self.algorithm.operations()(self.numbers.clone());
    }

    fn algorithm_input(&mut self, ui: &mut Ui) {
        let before = self.algorithm;

        ComboBox::from_id_salt("Algorithm")
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
                ui.selectable_value(
                    &mut self.algorithm,
                    Algorithm::Heap,
                    Algorithm::Heap.to_string(),
                );
                ui.selectable_value(
                    &mut self.algorithm,
                    Algorithm::Gnome,
                    Algorithm::Gnome.to_string(),
                );
                ui.selectable_value(
                    &mut self.algorithm,
                    Algorithm::Cocktail,
                    Algorithm::Cocktail.to_string(),
                );
                ui.selectable_value(
                    &mut self.algorithm,
                    Algorithm::OddEven,
                    Algorithm::OddEven.to_string(),
                );
                ui.selectable_value(
                    &mut self.algorithm,
                    Algorithm::Radix,
                    Algorithm::Radix.to_string(),
                );
            })
            .response
            .on_hover_text("Algorithm");

        if self.algorithm != before {
            self.reset_operations();
        }
    }

    fn options_panel(&mut self, ui: &mut Ui) {
        let randomize_icon = egui::include_image!("./images/randomize.png");
        let resume_icon = egui::include_image!("./images/resume.png");
        let pause_icon = egui::include_image!("./images/pause.png");
        let stop_icon = egui::include_image!("./images/stop.png");

        let is_stopped = self.state == SortingState::Idle;

        ui.horizontal(|ui| {
            ui.add_enabled_ui(is_stopped, |ui| self.algorithm_input(ui));

            if ui
                .add_enabled(is_stopped, Button::image(randomize_icon))
                .on_hover_text("Randomize")
                .clicked()
            {
                fastrand::shuffle(&mut self.numbers);
                self.reset_operations();
            }

            if self.state == SortingState::Running {
                if ui
                    .add(Button::image(pause_icon))
                    .on_hover_text("Pause")
                    .clicked()
                {
                    self.state = SortingState::Paused;
                }
            } else if ui
                .add(Button::image(resume_icon))
                .on_hover_text(if is_stopped { "Sort" } else { "Resume" })
                .clicked()
            {
                self.state = SortingState::Running;
            }

            if ui
                .add_enabled(!is_stopped, Button::image(stop_icon))
                .on_hover_text("Stop")
                .clicked()
            {
                self.state = SortingState::Idle;
                self.numbers = (1..=self.count).collect();
                self.active_elements = (None, None);
                self.reset_operations();
            };

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add_enabled(is_stopped, Slider::new(&mut self.count, 10..=1000))
                    .on_hover_text("Number of elements")
                    .changed()
                {
                    self.numbers = (1..=self.count).collect();
                    self.reset_operations();
                }

                if ui
                    .add(
                        DragValue::new(&mut self.speed)
                            .range(1..=100)
                            .prefix("Speed: ")
                            .suffix(" ops/s"),
                    )
                    .on_hover_text("Speed")
                    .changed()
                {
                    self.tick = Duration::from_millis(1000 / self.speed as u64);
                }
            });
        });
    }

    fn set_active_elements(&mut self, operation: Operation) {
        let theme = catppuccin_egui::MOCHA;

        self.active_elements = match operation {
            Operation::Compare(i, j) => {
                let element_1 = Some(HiglightedElement {
                    index: i,
                    color: theme.green,
                });

                let element_2 = Some(HiglightedElement {
                    index: j,
                    color: theme.green,
                });

                (element_1, element_2)
            }

            Operation::CompareToValue(i) => {
                let element = Some(HiglightedElement {
                    index: i,
                    color: theme.green,
                });

                (element, None)
            }

            Operation::Write(i, j) => {
                let element_1 = Some(HiglightedElement {
                    index: i,
                    color: theme.red,
                });

                let element_2 = Some(HiglightedElement {
                    index: j,
                    color: theme.peach,
                });

                (element_1, element_2)
            }

            Operation::WriteValue(i, _) => {
                let element = Some(HiglightedElement {
                    index: i,
                    color: theme.red,
                });

                (element, None)
            }

            Operation::Swap(i, j) => {
                let element_1 = Some(HiglightedElement {
                    index: i,
                    color: theme.blue,
                });

                let element_2 = Some(HiglightedElement {
                    index: j,
                    color: theme.blue,
                });

                (element_1, element_2)
            }
        }
    }

    fn sorting_panel(&mut self, ui: &mut Ui) {
        let mut next_operation = None;

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let is_running = self.state == SortingState::Running;

            #[cfg(not(target_arch = "wasm32"))]
            let instant = Instant::now();
            #[cfg(not(target_arch = "wasm32"))]
            let has_tick_elapsed = (instant - self.last_operation_instant) > self.tick;

            #[cfg(target_arch = "wasm32")]
            let instant = self.performance.now();
            #[cfg(target_arch = "wasm32")]
            let has_tick_elapsed =
                Duration::from_millis((instant - self.last_operation_instant) as u64) > self.tick;

            if is_running && has_tick_elapsed {
                next_operation = self.operations.next();
                self.last_operation_instant = instant;

                match next_operation {
                    Some(operation) => self.set_active_elements(operation),
                    None => {
                        self.state = SortingState::Idle;
                        self.reset_operations();
                        self.active_elements = (None, None);
                    }
                }
            }

            let area = response.rect;
            let bar_width = area.width() / self.count as f32;
            let bar_height_per_size = area.height() / self.count as f32;
            self.numbers.iter().enumerate().for_each(|(index, number)| {
                let left_x = area.min.x + bar_width * index as f32;
                let right_x = left_x + bar_width;
                let bottom_y = area.max.y;
                let top_y = area.max.y - bar_height_per_size * *number as f32;

                let bar =
                    Rect::from_two_pos(egui::pos2(left_x, bottom_y), egui::pos2(right_x, top_y));

                let mut color = catppuccin_egui::MOCHA.text;
                match self.active_elements {
                    (Some(element_1), Some(element_2)) => {
                        if element_1.index == index {
                            color = element_1.color;
                        } else if element_2.index == index {
                            color = element_2.color
                        }
                    }

                    (Some(element), None) => {
                        if element.index == index {
                            color = element.color;
                        }
                    }

                    (None, Some(_)) => unreachable!(),
                    (None, None) => {}
                }

                painter.rect_filled(bar, 0., color);
            });

            if let Some(operation) = next_operation {
                operation.apply(&mut self.numbers);
            }
        });
    }
}

impl App for AlgorithmVisualizer {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("options")
            .show_separator_line(false)
            .show(ctx, |ui| self.options_panel(ui));

        CentralPanel::default().show(ctx, |ui| self.sorting_panel(ui));

        ctx.request_repaint_after(self.tick);
    }
}
