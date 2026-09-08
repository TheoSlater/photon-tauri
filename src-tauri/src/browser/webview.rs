use super::BrowserResult;
use crate::platform;
#[cfg(target_os = "linux")]
use gtk::prelude::*;
use wry::{Rect, WebViewBuilder, WebViewRenderMode};

pub struct BrowserWebView {
    pub(crate) page: platform::PlatformPage,
}

impl BrowserWebView {
    pub fn create(
        window: &tauri::Window,
        host: &platform::BrowserHost,
        url: &str,
        navigation: impl Fn(String) -> bool + 'static,
        load: impl Fn(wry::PageLoadEvent, String) + 'static,
        title: impl Fn(String) + 'static,
    ) -> BrowserResult<Self> {
        let builder = WebViewBuilder::new()
            .with_render_mode(WebViewRenderMode::Composited)
            .with_focused(false)
            .with_navigation_handler(navigation)
            .with_on_page_load_handler(load)
            .with_document_title_changed_handler(title)
            .with_url(url);
        Ok(Self {
            page: platform::build_page(window, host, builder)?,
        })
    }

    pub fn navigate(&self, url: &str) -> BrowserResult<()> {
        self.page.webview.load_url(url)?;
        Ok(())
    }
    pub fn reload(&self) -> BrowserResult<()> {
        self.page.webview.reload()?;
        Ok(())
    }
    pub fn stop(&self) -> BrowserResult<()> {
        self.page.webview.evaluate_script("window.stop()")?;
        Ok(())
    }
    pub fn go_back(&self) -> BrowserResult<()> {
        self.page.webview.go_back()?;
        Ok(())
    }
    pub fn go_forward(&self) -> BrowserResult<()> {
        self.page.webview.go_forward()?;
        Ok(())
    }
    pub fn can_go_back(&self) -> BrowserResult<bool> {
        Ok(self.page.webview.can_go_back()?)
    }
    pub fn can_go_forward(&self) -> BrowserResult<bool> {
        Ok(self.page.webview.can_go_forward()?)
    }
    pub fn focus(&self) -> BrowserResult<()> {
        self.page.webview.focus()?;
        Ok(())
    }
    pub fn set_visible(&self, visible: bool) -> BrowserResult<()> {
        self.page.webview.set_visible(visible)?;
        #[cfg(target_os = "linux")]
        self.page.host.set_visible(visible);
        Ok(())
    }
    pub fn set_bounds(&self, bounds: Rect) -> BrowserResult<()> {
        self.page.webview.set_bounds(bounds)?;
        Ok(())
    }
}
