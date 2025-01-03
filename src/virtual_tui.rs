use taffy::prelude::{auto, length};

use crate::{sum_axis, tid, TaffyContainerUi, Tui, TuiBuilderLogic, TuiId};

/// Required parameters to correctly draw grid with virtual rows
pub struct VirtualGridRowHelperParams {
    /// Header row count that needs to be skipped in the grid
    pub header_row_count: u16,
    /// Data row count in the grid excluding any header rows
    pub row_count: usize,
}

/// Helper to draw grid with virtual rows
pub struct VirtualGridRowHelper;

/// Information about grid row that needs to be drawn
pub struct VirtualGridRow {
    /// Index of data from 0..row_count
    pub idx: usize,
    /// Row position in the grid
    ///
    /// Use [`GridRow::set_grid_row`] to retrieve closure that will set the style.
    pub grid_row: u16,
}

impl VirtualGridRow {
    /// Retrieve closure that can be used in `tui.mut_style(_)` to set grid_row parameter.
    pub fn grid_row_setter(&self) -> impl Fn(&mut taffy::Style) {
        let grid_row = self.grid_row;
        move |style: &mut taffy::Style| {
            style.grid_row = taffy::style_helpers::line(grid_row as i16);
        }
    }

    /// Retrieve closure that can be used to generate unique ids for elements in the row
    pub fn id_gen(&self) -> impl FnMut() -> TuiId {
        let idx = self.idx;
        let mut col_idx = 0;
        move || {
            col_idx += 1;
            tid(("cell", idx, col_idx))
        }
    }
}

/// Necessary information about grid item to draw correct virtual grid
pub struct VirtualGridRowInfo {
    /// Container value from any cell in the row
    pub container: TaffyContainerUi,
}

impl VirtualGridRowHelper {
    /// Show virtual grid rows.
    ///
    /// Closure receives information about grid row that needs to be drawn.
    /// Closure needs to return information about any cell that has full height in the grid row.
    /// All Rows should have equal heaight. One row will be used to estimate height of all rows.
    pub fn show<F>(params: VirtualGridRowHelperParams, tui: &mut Tui, mut draw_line: F)
    where
        F: FnMut(&mut Tui, VirtualGridRow) -> VirtualGridRowInfo,
    {
        let gap = match tui.current_style().gap.height {
            taffy::LengthPercentage::Length(length) => length,
            taffy::LengthPercentage::Percent(_) => {
                // TODO: Not supported yet
                0.
            }
        };

        let VirtualGridRowHelperParams {
            row_count,
            header_row_count,
        } = params;

        if row_count == 0 {
            return;
        }

        let mut grid_row = header_row_count + 1;

        // Draw first row for reference
        let info = draw_line(tui, VirtualGridRow { idx: 0, grid_row });

        if row_count == 1 {
            return;
        }

        let full_container = info.container.full_container_with(false);

        let row_height = full_container.height();
        let top_offset = (full_container.min - tui.current_viewport_content().min).y;
        let margin = info.container.layout().margin;

        let full_row_height = row_height + sum_axis(&margin).height + gap;

        let scroll_offset = -(tui.last_scroll_offset.y + top_offset);
        let visible_rect_size = tui.current_viewport().size().y;

        let visible_from =
            ((scroll_offset / full_row_height).floor().max(0.) as usize).clamp(1, row_count);

        let visible_to = ((((scroll_offset + visible_rect_size) / full_row_height).floor() + 1.)
            .max(0.) as usize)
            .clamp(visible_from, row_count);

        if visible_from > 1 {
            // Draw empty cell from 1..next_visible_from

            let row_count_to_hide = visible_from - 1;
            let height = (row_count_to_hide as f32) * full_row_height - gap;

            grid_row += 1;

            let size = taffy::Size {
                width: auto(),
                height: length(height),
            };

            tui.id("top_virtual")
                .style(taffy::Style {
                    min_size: size,
                    size,
                    max_size: size,
                    grid_row: taffy::style_helpers::line(grid_row as i16),
                    ..Default::default()
                })
                .add_empty();
        }

        if visible_from < visible_to {
            for row_idx in visible_from..visible_to {
                grid_row += 1;

                draw_line(
                    tui,
                    VirtualGridRow {
                        idx: row_idx,
                        grid_row,
                    },
                );
            }
        }

        if visible_to < row_count {
            // Draw empty cell from visible_to..row_count

            let row_count_to_hide = row_count - visible_to;
            let height = (row_count_to_hide as f32) * full_row_height - gap;

            grid_row += 1;

            let size = taffy::Size {
                width: auto(),
                height: length(height),
            };

            tui.id("bottom_virtual")
                .style(taffy::Style {
                    min_size: size,
                    size,
                    max_size: size,
                    grid_row: taffy::style_helpers::line(grid_row as i16),
                    ..Default::default()
                })
                .add_empty();
        }
    }
}
