pub mod commands;
pub mod events;
pub mod ids;
pub mod navigation;
pub mod page;
pub mod tab;
pub mod tabs;
pub mod viewport;
pub mod webview;
pub mod window;

pub type BrowserResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn start(window: &tauri::WebviewWindow, app: tauri::AppHandle) -> BrowserResult<()> {
    let browser = window::BrowserWindow::create(window.clone(), app)?;
    let window_id = browser.id;
    window::install(browser);
    events::install(window);
    eprintln!("photon: browser window ready ({window_id})");
    Ok(())
}
