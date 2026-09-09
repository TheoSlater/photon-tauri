use super::ids::TabId;
use super::BrowserResult;
use super::{
    navigation::NavigationRequest,
    tab::TabSnapshot,
    viewport::ViewportBounds,
    window::{self, BrowserStateSnapshot},
};
use std::sync::mpsc;

fn on_main<T, F>(app: tauri::AppHandle, operation: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce(&mut window::BrowserWindow) -> BrowserResult<T> + Send + 'static,
{
    let (send, receive) = mpsc::sync_channel(1);
    app.run_on_main_thread(move || {
        let result = window::with_active(|browser| operation(browser))
            .ok_or_else(|| "browser window is not initialized".into())
            .and_then(|result| result.map_err(|error| error.to_string()));
        let _ = send.send(result);
    })
    .map_err(|error| error.to_string())?;
    receive.recv().map_err(|error| error.to_string())?
}

#[tauri::command]
pub fn browser_create_tab(
    app: tauri::AppHandle,
    url: Option<String>,
) -> Result<TabSnapshot, String> {
    on_main(app, move |browser| {
        browser.create_tab(url.unwrap_or_else(|| window::DEFAULT_URL.into()))
    })
}

#[tauri::command]
pub fn browser_close_tab(app: tauri::AppHandle, tab_id: TabId) -> Result<(), String> {
    on_main(app, move |browser| browser.close_tab(tab_id))
}

#[tauri::command]
pub fn browser_activate_tab(app: tauri::AppHandle, tab_id: TabId) -> Result<(), String> {
    on_main(app, move |browser| browser.activate_tab(tab_id))
}

#[tauri::command]
pub fn browser_navigate(
    app: tauri::AppHandle,
    tab_id: TabId,
    request: NavigationRequest,
) -> Result<(), String> {
    on_main(app, move |browser| browser.navigate(tab_id, request))
}

#[tauri::command]
pub fn browser_reload(app: tauri::AppHandle, tab_id: TabId) -> Result<(), String> {
    on_main(app, move |browser| browser.reload(tab_id))
}

#[tauri::command]
pub fn browser_stop(app: tauri::AppHandle, tab_id: TabId) -> Result<(), String> {
    on_main(app, move |browser| browser.stop(tab_id))
}

#[tauri::command]
pub fn browser_back(app: tauri::AppHandle, tab_id: TabId) -> Result<(), String> {
    on_main(app, move |browser| browser.back(tab_id))
}

#[tauri::command]
pub fn browser_forward(app: tauri::AppHandle, tab_id: TabId) -> Result<(), String> {
    on_main(app, move |browser| browser.forward(tab_id))
}

#[tauri::command]
pub fn browser_get_state(app: tauri::AppHandle) -> Result<BrowserStateSnapshot, String> {
    on_main(app, |browser| Ok(browser.state()))
}

#[tauri::command]
pub fn browser_set_viewport(app: tauri::AppHandle, bounds: ViewportBounds) -> Result<(), String> {
    on_main(app, move |browser| browser.set_viewport(bounds))
}
