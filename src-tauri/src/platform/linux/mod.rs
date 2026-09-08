mod compatibility;

pub use compatibility::apply_pre_init_compatibility;

use gtk::prelude::*;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

use super::{BrowserHost, PlatformPage};

pub fn new_host(window: &tauri::WebviewWindow) -> Result<BrowserHost, wry::Error> {
    eprintln!("photon: backend = webkitgtk, render_mode = composited");
    // Wry applies Linux webview bounds through GtkFixed::put/size_allocate.
    // Keeping one fixed host gives every page the same window coordinate space.
    let root = gtk::Fixed::new();
    root.set_hexpand(true);
    root.set_vexpand(true);
    let vbox = window
        .default_vbox()
        .map_err(|error| wry::Error::Io(std::io::Error::other(error.to_string())))?;
    let frontend = vbox
        .children()
        .into_iter()
        .next()
        .ok_or_else(|| wry::Error::Io(std::io::Error::other("missing frontend webview")))?;
    vbox.remove(&frontend);

    let overlay = gtk::Overlay::new();
    overlay.add(&frontend);
    overlay.add_overlay(&root);
    root.set_halign(gtk::Align::Fill);
    root.set_valign(gtk::Align::Fill);
    root.show();
    overlay.show_all();
    vbox.pack_start(&overlay, true, true, 0);
    Ok(BrowserHost { root })
}

pub fn build_page<'a>(
    _window: &tauri::WebviewWindow,
    host: &BrowserHost,
    builder: WebViewBuilder<'a>,
) -> Result<PlatformPage, wry::Error> {
    let webview = builder.build_gtk(&host.root)?;
    Ok(PlatformPage { webview })
}
