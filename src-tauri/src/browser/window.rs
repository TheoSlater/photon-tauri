use super::BrowserResult;
use super::{
    events::{BrowserEvent, EventQueue, EVENT_NAME},
    ids::{BrowserWindowId, PageId, TabId},
    navigation::NavigationRequest,
    page::BrowserPage,
    tab::{Tab, TabSnapshot},
    tabs::TabManager,
};
use crate::platform;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};

pub const DEFAULT_URL: &str = "https://www.google.com";

pub struct BrowserWindow {
    pub id: BrowserWindowId,
    native: tauri::Window,
    host: platform::BrowserHost,
    tabs: TabManager,
    events: EventQueue,
    app: tauri::AppHandle,
}

#[derive(Clone, Debug, Serialize)]
pub struct BrowserStateSnapshot {
    pub active_tab_id: Option<TabId>,
    pub tabs: Vec<TabSnapshot>,
}

impl BrowserWindow {
    pub fn create(native: tauri::Window, app: tauri::AppHandle) -> BrowserResult<Self> {
        let host = platform::BrowserHost::new(&native)?;
        let mut browser = Self {
            id: BrowserWindowId::new(),
            native,
            host,
            tabs: TabManager::new(),
            events: Rc::new(RefCell::new(Vec::new())),
            app,
        };
        browser.create_tab(DEFAULT_URL.to_string())?;
        Ok(browser)
    }

    pub fn create_tab(&mut self, url: String) -> BrowserResult<TabSnapshot> {
        let tab_id = TabId::new();
        let page = BrowserPage::create(
            &self.native,
            &self.host,
            PageId::new(),
            url,
            tab_id,
            self.events.clone(),
        )?;
        let mut tab = Tab {
            id: tab_id,
            page,
            active: false,
        };
        tab.page.set_visible(false)?;
        self.tabs.insert(tab);
        self.events
            .borrow_mut()
            .push(BrowserEvent::TabCreated { tab_id });
        if self.tabs.active_id().is_none() {
            self.activate_tab(tab_id)?;
        }
        self.emit_events();
        Ok(self.tabs.get(tab_id).expect("inserted tab").snapshot())
    }

    pub fn close_tab(&mut self, tab_id: TabId) -> BrowserResult<()> {
        let was_active = self.tabs.active_id() == Some(tab_id);
        self.tabs.remove(tab_id).ok_or("unknown tab")?;
        self.events
            .borrow_mut()
            .push(BrowserEvent::TabClosed { tab_id });
        if was_active {
            if let Some(next) = self.tabs.active_id() {
                self.activate_tab(next)?;
            }
        }
        self.emit_events();
        Ok(())
    }

    pub fn activate_tab(&mut self, tab_id: TabId) -> BrowserResult<()> {
        let changed = self.tabs.active_id() != Some(tab_id);
        if changed {
            if let Some(old_id) = self.tabs.active_id() {
                if let Some(old) = self.tabs.get_mut(old_id) {
                    old.page.set_visible(false)?;
                }
            }
            self.tabs.activate(tab_id)?;
        }
        let bounds = BrowserPage::full_bounds(&self.native)?;
        let tab = self.tabs.get_mut(tab_id).ok_or("unknown tab")?;
        tab.page.set_bounds(bounds)?;
        tab.page.set_visible(true)?;
        tab.page.focus()?;
        tab.page.refresh_history_state()?;
        if changed {
            self.events
                .borrow_mut()
                .push(BrowserEvent::TabActivated { tab_id });
        }
        self.emit_events();
        Ok(())
    }

    pub fn navigate(&mut self, tab_id: TabId, request: NavigationRequest) -> BrowserResult<()> {
        let tab = self.tabs.get_mut(tab_id).ok_or("unknown tab")?;
        tab.page.navigate(request)?;
        tab.page.refresh_history_state()?;
        self.emit_events();
        Ok(())
    }

    pub fn reload(&mut self, tab_id: TabId) -> BrowserResult<()> {
        let tab = self.tabs.get_mut(tab_id).ok_or("unknown tab")?;
        tab.page.reload()?;
        tab.page.refresh_history_state()
    }
    pub fn stop(&mut self, tab_id: TabId) -> BrowserResult<()> {
        self.tabs.get(tab_id).ok_or("unknown tab")?.page.stop()
    }
    pub fn back(&mut self, tab_id: TabId) -> BrowserResult<()> {
        let tab = self.tabs.get_mut(tab_id).ok_or("unknown tab")?;
        tab.page.go_back()?;
        tab.page.refresh_history_state()
    }
    pub fn forward(&mut self, tab_id: TabId) -> BrowserResult<()> {
        let tab = self.tabs.get_mut(tab_id).ok_or("unknown tab")?;
        tab.page.go_forward()?;
        tab.page.refresh_history_state()
    }

    pub fn state(&self) -> BrowserStateSnapshot {
        BrowserStateSnapshot {
            active_tab_id: self.tabs.active_id(),
            tabs: self.tabs.snapshots(),
        }
    }

    pub fn resize(&mut self) -> BrowserResult<()> {
        if let Some(tab_id) = self.tabs.active_id() {
            let bounds = BrowserPage::full_bounds(&self.native)?;
            if let Some(tab) = self.tabs.get(tab_id) {
                tab.page.set_bounds(bounds)?;
            }
        }
        Ok(())
    }

    fn emit_events(&self) {
        for event in self.events.borrow_mut().drain(..) {
            let _ = tauri::Emitter::emit(&self.app, EVENT_NAME, event);
        }
    }
}

thread_local! {
    static ACTIVE: RefCell<Option<BrowserWindow>> = const { RefCell::new(None) };
}

pub fn install(browser: BrowserWindow) {
    ACTIVE.with(|active| active.borrow_mut().replace(browser));
}

pub fn with_active<F, R>(action: F) -> Option<R>
where
    F: FnOnce(&mut BrowserWindow) -> R,
{
    ACTIVE.with(|active| active.borrow_mut().as_mut().map(action))
}

pub fn drop_active() {
    ACTIVE.with(|active| {
        if active.borrow_mut().take().is_some() {
            eprintln!("photon: browser window destroyed");
        }
    });
}
