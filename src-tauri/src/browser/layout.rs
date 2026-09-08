use wry::{dpi::LogicalPosition, dpi::LogicalSize, Rect};

// Keep these values paired with src/layout.ts while the shell is intentionally
// small. The native Wry page must use the same coordinates as the React frame.
pub const TOP_BAR_HEIGHT: f64 = 40.0;
pub const PAGE_PADDING: f64 = 4.0;

pub fn page_bounds(logical_width: f64, logical_height: f64) -> Rect {
    Rect {
        position: LogicalPosition::new(PAGE_PADDING, TOP_BAR_HEIGHT + PAGE_PADDING).into(),
        size: LogicalSize::new(
            (logical_width - PAGE_PADDING * 2.0).max(0.0),
            (logical_height - TOP_BAR_HEIGHT - PAGE_PADDING * 2.0).max(0.0),
        )
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_bounds_leave_top_bar_and_padding() {
        let bounds = page_bounds(1000.0, 700.0);

        assert_eq!(bounds.position, LogicalPosition::new(4.0, 44.0).into());
        assert_eq!(bounds.size, LogicalSize::new(992.0, 652.0).into());
    }
}
