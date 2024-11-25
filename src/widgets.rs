use egui::Ui;

use crate::{TuiBuilder, TuiBuilderLogic, TuiWidget};

/// Separator that correctly grows in tui environment in both axis
#[derive(Default)]
pub struct TaffySeparator {
    is_horizontal_line: Option<bool>,
    separator: egui::Separator,
}

impl TaffySeparator {
    /// Draw this separator line vertically
    pub fn vertical(mut self) -> Self {
        self.is_horizontal_line = Some(false);
        self.separator = self.separator.vertical();
        self
    }
}

impl egui::Widget for TaffySeparator {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.add(self.separator)
    }
}

impl TuiWidget for TaffySeparator {
    type Response = egui::Response;

    fn taffy_ui(self, tui: TuiBuilder) -> Self::Response {
        let tui = tui.mut_style(|style| {
            style.min_size = taffy::Size {
                width: taffy::Dimension::Length(0.),
                height: taffy::Dimension::Length(0.),
            };
        });

        let is_horizontal_line = self.is_horizontal_line;

        tui.ui_add_manual(
            |ui| ui.add(self),
            |mut space, ui| {
                let is_horizontal_line =
                    is_horizontal_line.unwrap_or_else(|| !ui.layout().main_dir().is_horizontal());
                if let Some(size) = space.intrinsic_size.as_mut() {
                    match is_horizontal_line {
                        true => {
                            size.x = 0.;
                            space.infinite.x = true;
                        }
                        false => {
                            size.y = 0.;
                            space.infinite.y = true;
                        }
                    }
                }

                space
            },
        )
    }
}
