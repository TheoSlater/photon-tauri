use super::{BrowserHost, PlatformPage};
use wry::WebViewBuilder;

pub fn new_host(_window: &tauri::Window) -> Result<BrowserHost, wry::Error> {
    eprintln!("photon: backend = webview2, render_mode = composited");
    Ok(BrowserHost {})
}

pub fn build_page<'a>(
    window: &tauri::Window,
    _host: &BrowserHost,
    builder: WebViewBuilder<'a>,
) -> Result<PlatformPage, wry::Error> {
    Ok(PlatformPage {
        webview: builder.build(window)?,
    })
}
