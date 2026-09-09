use super::BrowserResult;
use super::{
    events::BrowserEvent, ids::*, navigation::NavigationRequest, overlays::OverlayRegistry,
    viewport::ViewportBounds,
};
use crate::platform;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wry::Rect;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PageState {
    pub url: Option<String>,
    pub title: Option<String>,
    pub loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    #[serde(skip)]
    pub visible: bool,
}

pub struct BrowserPage {
    pub id: PageId,
    state: Rc<RefCell<PageState>>,
    webview: Option<super::webview::BrowserWebView>,
    viewport: Rc<RefCell<ViewportBounds>>,
}

impl BrowserPage {
    pub fn create(
        window: &tauri::WebviewWindow,
        host: &platform::BrowserHost,
        id: PageId,
        url: String,
        tab_id: TabId,
        events: super::events::EventQueue,
        initial_viewport: ViewportBounds,
        overlays: OverlayRegistry,
    ) -> BrowserResult<Self> {
        let state = Rc::new(RefCell::new(PageState {
            url: Some(url.clone()),
            ..Default::default()
        }));
        let navigation_state = Rc::clone(&state);
        let load_state = Rc::clone(&state);
        let page_events = events.clone();
        let title_state = Rc::clone(&state);
        let title_events = events.clone();
        let viewport = Rc::new(RefCell::new(initial_viewport));
        let hit_test_viewport = Rc::clone(&viewport);
        let hit_test_overlays = overlays;
        let webview = super::webview::BrowserWebView::create(
            window,
            host,
            initial_viewport.to_rect(),
            &url,
            move |url| {
                navigation_state.borrow_mut().url = Some(url.clone());
                true
            },
            move |load, url| {
                let mut state = load_state.borrow_mut();
                match load {
                    wry::PageLoadEvent::Started => {
                        state.loading = true;
                        page_events
                            .borrow_mut()
                            .push(BrowserEvent::NavigationStarted { tab_id, url });
                    }
                    wry::PageLoadEvent::Finished => {
                        state.loading = false;
                        state.url = Some(url.clone());
                        page_events
                            .borrow_mut()
                            .push(BrowserEvent::NavigationFinished { tab_id, url });
                    }
                }
            },
            move |title| {
                title_state.borrow_mut().title = Some(title.clone());
                title_events
                    .borrow_mut()
                    .push(BrowserEvent::TitleChanged { tab_id, title });
            },
            move |x, y| {
                let viewport = *hit_test_viewport.borrow();
                if hit_test_overlays
                    .hit_test(viewport.x + x, viewport.y + y)
                    .is_some()
                {
                    return false;
                }
                !matches!(
                    hit_test_overlays.outside_mode(),
                    Some(
                        super::overlays::OverlayInteractionMode::Dismiss
                            | super::overlays::OverlayInteractionMode::Modal
                    )
                )
            },
        )?;
        Ok(Self {
            id,
            state,
            webview: Some(webview),
            viewport,
        })
    }

    #[cfg(test)]
    pub fn placeholder(id: PageId, url: Option<String>) -> Self {
        Self {
            id,
            state: Rc::new(RefCell::new(PageState {
                url,
                ..Default::default()
            })),
            webview: None,
            viewport: Rc::new(RefCell::new(ViewportBounds::default())),
        }
    }

    pub fn state(&self) -> PageState {
        self.state.borrow().clone()
    }

    pub fn navigate(&mut self, request: NavigationRequest) -> BrowserResult<()> {
        let page = self.webview.as_ref().ok_or("page has no native webview")?;
        page.navigate(&request.url)?;
        self.state.borrow_mut().url = Some(request.url);
        Ok(())
    }

    pub fn reload(&self) -> BrowserResult<()> {
        self.webview
            .as_ref()
            .ok_or("page has no native webview")?
            .reload()?;
        Ok(())
    }

    pub fn stop(&self) -> BrowserResult<()> {
        self.webview
            .as_ref()
            .ok_or("page has no native webview")?
            .stop()?;
        Ok(())
    }

    pub fn go_back(&self) -> BrowserResult<()> {
        self.webview
            .as_ref()
            .ok_or("page has no native webview")?
            .go_back()?;
        Ok(())
    }

    pub fn go_forward(&self) -> BrowserResult<()> {
        self.webview
            .as_ref()
            .ok_or("page has no native webview")?
            .go_forward()?;
        Ok(())
    }

    pub fn focus(&self) -> BrowserResult<()> {
        self.webview
            .as_ref()
            .ok_or("page has no native webview")?
            .focus()?;
        Ok(())
    }

    pub fn set_visible(&mut self, visible: bool) -> BrowserResult<()> {
        if let Some(webview) = &self.webview {
            webview.set_visible(visible)?;
        }
        self.state.borrow_mut().visible = visible;
        Ok(())
    }

    pub fn set_bounds(&self, bounds: Rect) -> BrowserResult<()> {
        if let Some(webview) = &self.webview {
            webview.set_bounds(bounds)?;
        }
        Ok(())
    }

    pub fn set_viewport(&mut self, bounds: ViewportBounds) -> BrowserResult<()> {
        *self.viewport.borrow_mut() = bounds;
        self.set_bounds(bounds.to_rect())?;
        self.set_visible(bounds.has_area())
    }

    pub fn refresh_history_state(&mut self) -> BrowserResult<()> {
        if let Some(webview) = &self.webview {
            let mut state = self.state.borrow_mut();
            state.can_go_back = webview.can_go_back()?;
            state.can_go_forward = webview.can_go_forward()?;
        }
        Ok(())
    }
}
