#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct BrowserHost {
    #[cfg(target_os = "linux")]
    pub(crate) root: gtk::Fixed,
}

pub struct PlatformPage {
    pub(crate) webview: wry::WebView,
}

#[cfg(target_os = "linux")]
pub fn apply_pre_init_compatibility() {
    linux::apply_pre_init_compatibility();
}

impl BrowserHost {
    pub fn new(window: &tauri::WebviewWindow) -> Result<Self, wry::Error> {
        #[cfg(target_os = "linux")]
        return linux::new_host(window);
        #[cfg(target_os = "macos")]
        return macos::new_host(window);
        #[cfg(target_os = "windows")]
        return windows::new_host(window);
    }
}

pub fn build_page<'a>(
    window: &tauri::WebviewWindow,
    host: &BrowserHost,
    builder: wry::WebViewBuilder<'a>,
) -> Result<PlatformPage, wry::Error> {
    #[cfg(target_os = "linux")]
    return linux::build_page(window, host, builder);
    #[cfg(target_os = "macos")]
    return macos::build_page(window, host, builder);
    #[cfg(target_os = "windows")]
    return windows::build_page(window, host, builder);
}
