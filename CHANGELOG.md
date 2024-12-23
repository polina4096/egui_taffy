# Changelog

## 0.5

Release adds support for scroll area, handling overflow css style.

* Added overflow demo to examples for all overflow settings: Visible, Clip, Hidden, Scroll,.
* Added support to hide or clip overflowing content using `taffy::Overflow::Clip`, `taffy::Overflow::Hidden`.
* Automatically add `egui::ScrollArea` when overflow: `taffy::Overflow:Scroll` is set.
* `add_with_border` now sets border_size value in taffy::Style if border size was set to default value (`Rect::zero()`).

## 0.4

* Support egui 0.30

## 0.3

Release adds support for more granular interaction with underlying `egui::Ui`.
When creating child elements you can provide additional settings that are passed to `egui::UiBuilder`.
(`egui::Layout`, `egui::Style`, `egui::TextWrapMode`, Disable descendant ui).

* Removed lifetime requirement for `Tui` (previously `Tui<'a>`).
* Added shorthand function for adding label with "strong" coloring. `tui.strong("label");`
* Added helper function to set wrap mode for child layout `tui.wrap_mode(egui::TextWrapMode::...).add(|tui| ...)`.
* Added methods to set up child element egui Ui style and layout: `tui.layout(egui::Layout::default()).egui_style(custom_egui_style).add(|tui| ...)`

## 0.2.1

* Correctly support child element/node disabling (egui::Ui disable).

## 0.2

* Taffy updated to 0.7.
* Added support for classic buttons and selectable buttons.
* Added information to README about text wrapping.

## 0.1

Initial functionality
