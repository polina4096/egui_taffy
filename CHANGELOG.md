# Changelog

## 0.3

* Removed lifetime requirement for `Tui` (previously `Tui<'a>`).
* Added shorthand function for adding label with "strong" coloring. `tui.strong("label");`
* Added helper function to set wrap mode for child layout `tui.wrap_mode(egui::TextWrapMode::...).add(|tui| ...)`.

## 0.2.1

* Correctly support child element/node disabling (egui::Ui disable).

## 0.2

* Taffy updated to 0.7.
* Added support for classic buttons and selectable buttons.
* Added information to README about text wrapping.

## 0.1

Initial functionality
