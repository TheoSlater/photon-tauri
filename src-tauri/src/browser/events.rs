use super::ids::TabId;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};

pub const EVENT_NAME: &str = "photon://browser-event";
pub type EventQueue = Rc<RefCell<Vec<BrowserEvent>>>;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BrowserEvent {
    TabCreated { tab_id: TabId },
    TabClosed { tab_id: TabId },
    TabActivated { tab_id: TabId },
    NavigationStarted { tab_id: TabId, url: String },
    NavigationFinished { tab_id: TabId, url: String },
    TitleChanged { tab_id: TabId, title: String },
}

pub fn install(window: &tauri::WebviewWindow) {
    window.on_window_event(|event| {
        if matches!(event, tauri::WindowEvent::CloseRequested { .. }) {
            super::window::drop_active();
        }
    });
}
