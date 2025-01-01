use egui::{Align, Ui};
use taffy::prelude::length;

use crate::{TuiBuilder, TuiBuilderLogic, TuiWidget};

/// Separator that correctly grows in tui environment in both axis
///
/// Determines draw dimension based on parent node taffy::Style flex direction.
#[derive(Default)]
pub struct TaffySeparator {
    separator: egui::Separator,
}

impl egui::Widget for TaffySeparator {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.add(self.separator)
    }
}

impl TuiWidget for TaffySeparator {
    type Response = egui::Response;

    fn taffy_ui(mut self, mut tui: TuiBuilder) -> Self::Response {
        let flex_direction = tui.builder_tui().current_style().flex_direction;

        let stroke = tui
            .builder_tui()
            .egui_ui()
            .visuals()
            .widgets
            .noninteractive
            .bg_stroke;

        tui = tui.mut_style(|style| {
            style.align_self = Some(taffy::AlignItems::Stretch);
            style.min_size = length(stroke.width);
            style.padding = length(3.);
        });

        let is_horizontal = match flex_direction {
            taffy::FlexDirection::Row => false,
            taffy::FlexDirection::Column => true,
            taffy::FlexDirection::RowReverse => false,
            taffy::FlexDirection::ColumnReverse => true,
        };

        let layout = match is_horizontal {
            true => {
                self.separator = self.separator.horizontal();
                egui::Layout::top_down(Align::Center)
            }
            false => {
                self.separator = self.separator.vertical();
                egui::Layout::left_to_right(Align::Center)
            }
        };

        let mut response = None;
        tui.egui_layout(layout).add_with_background_ui(
            |ui, rect| {
                let _ = rect;
                response = Some(ui.add(self.separator));
            },
            |tui| {
                let _ = tui;
            },
        );
        response.unwrap()
    }
}
