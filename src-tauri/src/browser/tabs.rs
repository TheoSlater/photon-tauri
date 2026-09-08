use super::{
    ids::TabId,
    tab::{Tab, TabSnapshot},
};

pub struct TabManager {
    tabs: Vec<Tab>,
    active: Option<TabId>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active: None,
        }
    }

    pub fn insert(&mut self, tab: Tab) {
        if tab.active {
            self.active = Some(tab.id);
        }
        self.tabs.push(tab);
    }

    pub fn remove(&mut self, id: TabId) -> Option<Tab> {
        let index = self.tabs.iter().position(|tab| tab.id == id)?;
        let removed = self.tabs.remove(index);
        if self.active == Some(id) {
            self.active = self
                .tabs
                .get(index)
                .or_else(|| self.tabs.last())
                .map(|tab| tab.id);
            for tab in &mut self.tabs {
                tab.active = Some(tab.id) == self.active;
            }
        }
        Some(removed)
    }

    pub fn activate(&mut self, id: TabId) -> Result<(), String> {
        if !self.tabs.iter().any(|tab| tab.id == id) {
            return Err("unknown tab".into());
        }
        self.active = Some(id);
        for tab in &mut self.tabs {
            tab.active = tab.id == id;
        }
        Ok(())
    }

    pub fn active_id(&self) -> Option<TabId> {
        self.active
    }
    pub fn get(&self, id: TabId) -> Option<&Tab> {
        self.tabs.iter().find(|tab| tab.id == id)
    }
    pub fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|tab| tab.id == id)
    }
    pub fn snapshots(&self) -> Vec<TabSnapshot> {
        self.tabs.iter().map(Tab::snapshot).collect()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::{ids::PageId, page::BrowserPage};

    fn tab(active: bool) -> Tab {
        Tab {
            id: TabId::new(),
            page: BrowserPage::placeholder(PageId::new(), None),
            active,
        }
    }

    #[test]
    fn tab_lifecycle_and_snapshots() {
        let mut tabs = TabManager::new();
        let first = tab(true);
        let first_id = first.id;
        tabs.insert(first);
        assert_eq!(tabs.active_id(), Some(first_id));
        let second = tab(false);
        let second_id = second.id;
        tabs.insert(second);
        tabs.activate(second_id).unwrap();
        assert_eq!(tabs.active_id(), Some(second_id));
        tabs.remove(first_id);
        assert_eq!(tabs.snapshots().len(), 1);
        tabs.remove(second_id);
        assert_eq!(tabs.active_id(), None);
        assert!(tabs.activate(TabId::new()).is_err());
    }
}
