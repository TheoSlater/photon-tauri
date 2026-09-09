use super::viewport::ViewportBounds;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OverlayInteractionMode {
    Passthrough,
    Dismiss,
    Modal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OverlayRegion {
    pub id: String,
    pub bounds: ViewportBounds,
    pub order: i32,
    pub interaction: OverlayInteractionMode,
    #[serde(default)]
    pub dismiss_on_outside_press: bool,
}

#[derive(Clone, Default)]
pub struct OverlayRegistry {
    regions: Rc<RefCell<BTreeMap<String, OverlayRegion>>>,
}

impl OverlayRegistry {
    pub fn register(&self, region: OverlayRegion) -> Result<(), &'static str> {
        if region.id.trim().is_empty() {
            return Err("overlay id cannot be empty");
        }
        if !region.bounds.is_valid() {
            return Err("invalid overlay bounds");
        }
        self.regions.borrow_mut().insert(region.id.clone(), region);
        Ok(())
    }

    pub fn unregister(&self, id: &str) {
        self.regions.borrow_mut().remove(id);
    }

    pub fn regions(&self) -> Vec<OverlayRegion> {
        self.regions.borrow().values().cloned().collect()
    }

    pub fn hit_test(&self, x: f64, y: f64) -> Option<String> {
        self.regions
            .borrow()
            .values()
            .filter(|region| region.bounds.contains(x, y))
            .max_by_key(|region| region.order)
            .map(|region| region.id.clone())
    }

    pub fn outside_mode(&self) -> Option<OverlayInteractionMode> {
        let regions = self.regions.borrow();
        if regions
            .values()
            .filter(|region| region.bounds.has_area())
            .any(|region| region.interaction == OverlayInteractionMode::Modal)
        {
            Some(OverlayInteractionMode::Modal)
        } else if regions
            .values()
            .filter(|region| region.bounds.has_area())
            .any(|region| region.interaction == OverlayInteractionMode::Dismiss)
        {
            Some(OverlayInteractionMode::Dismiss)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OverlayInteractionMode, OverlayRegion, OverlayRegistry};
    use crate::browser::viewport::ViewportBounds;

    fn region(id: &str, order: i32, x: f64, interaction: OverlayInteractionMode) -> OverlayRegion {
        OverlayRegion {
            id: id.into(),
            bounds: ViewportBounds {
                x,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            order,
            interaction,
            dismiss_on_outside_press: false,
        }
    }

    #[test]
    fn topmost_interactive_region_wins() {
        let registry = OverlayRegistry::default();
        registry
            .register(region("lower", 1, 0.0, OverlayInteractionMode::Passthrough))
            .unwrap();
        registry
            .register(region("upper", 2, 0.0, OverlayInteractionMode::Modal))
            .unwrap();

        assert_eq!(registry.hit_test(50.0, 50.0).as_deref(), Some("upper"));
    }

    #[test]
    fn unregister_removes_hit_region() {
        let registry = OverlayRegistry::default();
        registry
            .register(region("test", 1, 0.0, OverlayInteractionMode::Passthrough))
            .unwrap();
        registry.unregister("test");

        assert_eq!(registry.hit_test(50.0, 50.0), None);
    }

    #[test]
    fn modal_policy_wins_over_dismiss_policy() {
        let registry = OverlayRegistry::default();
        registry
            .register(region("menu", 10, 0.0, OverlayInteractionMode::Dismiss))
            .unwrap();
        registry
            .register(region("dialog", 1, 200.0, OverlayInteractionMode::Modal))
            .unwrap();

        assert_eq!(registry.outside_mode(), Some(OverlayInteractionMode::Modal));
    }
}
