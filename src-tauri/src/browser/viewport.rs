use serde::{Deserialize, Serialize};
use wry::{dpi::LogicalPosition, dpi::LogicalSize, Rect};

const MAX_VIEWPORT_VALUE: f64 = 100_000.0;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ViewportBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl ViewportBounds {
    pub fn is_valid(self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.x.abs() <= MAX_VIEWPORT_VALUE
            && self.y.abs() <= MAX_VIEWPORT_VALUE
            && self.width >= 0.0
            && self.height >= 0.0
            && self.width <= MAX_VIEWPORT_VALUE
            && self.height <= MAX_VIEWPORT_VALUE
    }

    pub fn has_area(self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    pub fn to_rect(self) -> Rect {
        Rect {
            position: LogicalPosition::new(self.x, self.y).into(),
            size: LogicalSize::new(self.width, self.height).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ViewportBounds;

    #[test]
    fn rejects_invalid_geometry() {
        assert!(!ViewportBounds {
            width: -1.0,
            ..Default::default()
        }
        .is_valid());
        assert!(!ViewportBounds {
            height: f64::NAN,
            ..Default::default()
        }
        .is_valid());
        assert!(!ViewportBounds {
            width: 100_001.0,
            ..Default::default()
        }
        .is_valid());
    }

    #[test]
    fn zero_viewport_has_no_area() {
        assert!(!ViewportBounds::default().has_area());
    }
}
