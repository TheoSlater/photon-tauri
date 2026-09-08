mod compatibility;

pub use compatibility::apply_pre_init_compatibility;

use gtk::prelude::*;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

use super::{BrowserHost, PlatformPage};

pub fn new_host(window: &tauri::Window) -> Result<BrowserHost, wry::Error> {
    eprintln!("photon: backend = webkitgtk, render_mode = composited");
    // Wry applies Linux webview bounds through GtkFixed::put/size_allocate.
    // Keeping one fixed host gives every page the same window coordinate space.
    let root = gtk::Fixed::new();
    root.set_hexpand(true);
    root.set_vexpand(true);
    window
        .default_vbox()
        .map_err(|error| wry::Error::Io(std::io::Error::other(error.to_string())))?
        .pack_start(&root, true, true, 0);
    root.show();
    Ok(BrowserHost { root })
}

pub fn build_page<'a>(
    _window: &tauri::Window,
    host: &BrowserHost,
    builder: WebViewBuilder<'a>,
) -> Result<PlatformPage, wry::Error> {
    let webview = builder.build_gtk(&host.root)?;
    Ok(PlatformPage { webview })
}
