#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct BrowserHost {
    #[cfg(target_os = "linux")]
    pub(crate) root: gtk::Overlay,
}

pub struct PlatformPage {
    pub(crate) webview: wry::WebView,
    #[cfg(target_os = "linux")]
    pub(crate) host: gtk::Overlay,
}

impl BrowserHost {
    pub fn new(window: &tauri::Window) -> Result<Self, wry::Error> {
        #[cfg(target_os = "linux")]
        return linux::new_host(window);
        #[cfg(target_os = "macos")]
        return macos::new_host(window);
        #[cfg(target_os = "windows")]
        return windows::new_host(window);
    }
}

pub fn build_page<'a>(
    window: &tauri::Window,
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
