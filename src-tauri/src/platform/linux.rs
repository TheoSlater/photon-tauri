use gtk::prelude::*;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

use super::{BrowserHost, PlatformPage};

pub fn new_host(window: &tauri::Window) -> Result<BrowserHost, wry::Error> {
    eprintln!("photon: backend = webkitgtk, render_mode = composited");
    let root = gtk::Overlay::new();
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
    let page_host = gtk::Overlay::new();
    page_host.set_hexpand(true);
    page_host.set_vexpand(true);
    host.root.add_overlay(&page_host);
    let webview = builder.build_gtk(&page_host)?;
    page_host.show_all();
    Ok(PlatformPage {
        webview,
        host: page_host,
    })
}
