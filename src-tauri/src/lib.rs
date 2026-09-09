mod browser;
mod platform;
mod window;

use browser::commands::*;

#[cfg(target_os = "linux")]
pub fn apply_pre_init_compatibility() {
    platform::apply_pre_init_compatibility();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    eprintln!("photon: starting");
    tauri::Builder::default()
        .setup(window::setup::install)
        .invoke_handler(tauri::generate_handler![
            browser_create_tab,
            browser_close_tab,
            browser_activate_tab,
            browser_navigate,
            browser_reload,
            browser_stop,
            browser_back,
            browser_forward,
            browser_get_state,
            browser_set_viewport,
            browser_register_overlay,
            browser_unregister_overlay,
        ])
        .run(tauri::generate_context!())
        .expect("photon: failed to run");
}
