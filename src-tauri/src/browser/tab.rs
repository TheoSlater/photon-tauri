use super::{
    ids::{PageId, TabId},
    page::{BrowserPage, PageState},
};
use serde::Serialize;

pub struct Tab {
    pub id: TabId,
    pub page: BrowserPage,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct TabSnapshot {
    pub id: TabId,
    pub page_id: PageId,
    pub url: Option<String>,
    pub title: Option<String>,
    pub loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub active: bool,
}

impl Tab {
    pub fn snapshot(&self) -> TabSnapshot {
        let PageState {
            url,
            title,
            loading,
            can_go_back,
            can_go_forward,
            ..
        } = self.page.state();
        TabSnapshot {
            id: self.id,
            page_id: self.page.id,
            url,
            title,
            loading,
            can_go_back,
            can_go_forward,
            active: self.active,
        }
    }
}
