use super::BrowserResult;
use crate::platform;
use wry::{Rect, WebViewBuilder, WebViewRenderMode};

#[cfg(target_os = "windows")]
use wry::{Theme, WebViewBuilderExtWindows};

const SYSTEM_COLOR_SCHEME_SCRIPT: &str = r#"
  (() => {
    const apply = () => {
      if (document.documentElement) {
        document.documentElement.style.colorScheme = "light dark";
      }
    };
    if (document.documentElement) {
      apply();
    } else {
      document.addEventListener("DOMContentLoaded", apply, { once: true });
    }
  })();
"#;

pub struct BrowserWebView {
    pub(crate) page: platform::PlatformPage,
}

impl BrowserWebView {
    pub fn create(
        window: &tauri::WebviewWindow,
        host: &platform::BrowserHost,
        bounds: Rect,
        url: &str,
        navigation: impl Fn(String) -> bool + 'static,
        load: impl Fn(wry::PageLoadEvent, String) + 'static,
        title: impl Fn(String) + 'static,
    ) -> BrowserResult<Self> {
        let builder = WebViewBuilder::new()
            .with_render_mode(WebViewRenderMode::Composited)
            .with_bounds(bounds)
            .with_focused(false)
            .with_initialization_script(SYSTEM_COLOR_SCHEME_SCRIPT)
            .with_navigation_handler(navigation)
            .with_on_page_load_handler(load)
            .with_document_title_changed_handler(title)
            .with_url(url);
        #[cfg(target_os = "windows")]
        let builder = builder.with_theme(Theme::Auto);
        Ok(Self {
            page: platform::build_page(window, host, builder, bounds)?,
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
        Ok(())
    }
    pub fn set_bounds(&self, bounds: Rect) -> BrowserResult<()> {
        self.page.set_bounds(bounds)?;
        Ok(())
    }
}
